# Vault token renewal

CodeTether maintains renewable Vault token leases while the owning secrets
manager is alive. This applies to Windows user tokens loaded through `VAULT_TOKEN`
as well as configured/Kubernetes-authenticated managers. Clones share one monitor.

The monitor reads `auth/token/lookup-self`, then renews through
`auth/token/renew-self` approximately halfway through the reported TTL. Long
leases are checked at most one hour later. Every renewal response supplies the
next lease duration; a fixed lifetime is not assumed. The token string normally
stays unchanged, so the Windows user environment does not need rewriting.

Lookup denial does not automatically imply expiry: the monitor tries renew-self
directly, because policies may allow renewal without lookup. Transport/server
failures retry with bounded backoff and request deadlines. HTTP rejection stops
the loop with an explicit warning rather than spinning or claiming success.

## Requirements and limitations

- The token must be renewable, without a finite use count, and its policy must
  permit self renewal. The standard Vault default policy normally supplies this.
- Non-expiring tokens need no renewal. Non-renewable or use-limited tokens are
  identified without repeatedly consuming API requests.
- Renewal cannot revive an expired/revoked token or bypass maximum TTL limits.
  Reauthenticate or supply a new renewable token in that case. Kubernetes clients
  retain their existing reauthentication-on-rejection behavior.
- Closing CodeTether stops renewal; this is not a Windows background service.
- The native OCR readiness/desktop subprocesses do not initialize Vault or start
  competing credential-renewal loops.

Renewal logs contain TTL, renewability and failure categories, never token values,
accessors, authentication response bodies or secret data. A failed permission
check is not labeled an expired token without additional evidence.

Mocked HTTP tests cover renew-self requests, TTL scheduling, clone/drop lifetime,
retry after server failure, missing lookup permission, terminal rejection and
client replacement. These tests do not establish live renewal for a particular
deployment's token policy.