# Official Monarch connector

The native tool registry includes `monarch`, including when no LLM provider is
configured. No Python bridge, private GraphQL API, copied browser token, Monarch
password, or per-project MCP configuration is needed. Existing installations need
a normal CodeTether update containing this integration; editing source does not
upgrade running instances.

## Authentication

Use CodeTether's configured secrets backend and normal auth command:

```sh
codetether auth monarch login
codetether auth monarch status
```

Bare `codetether auth monarch` also starts login. The backend must already be
configured and authenticated through the existing CodeTether Vault workflow. A
backend outage or permission denial is an error, not an empty credential store.
The connector neither changes backend policies nor imports legacy credentials.

Open the printed official Monarch authorization URL in a full browser, sign in,
and review the read-only grant. The command waits up to five minutes for a
loopback callback. Never paste passwords, tokens, or callback URLs into chat.
For a remote CLI, forward the actual port printed in `redirect_uri` from the
browser's machine before opening the authorization URL:

```sh
ssh -N -L PORT:127.0.0.1:PORT USER@HOST
```

Replace `PORT` and `USER@HOST` with the callback port and host. Restart login for
a fresh authorization URL if it times out. Script-only browser renderers may
not run Monarch's JavaScript sign-in page.

Status returns `credentials_present`, `backend: "configured_vault"`, and
`mode: "read_only"`. Presence does **not** establish that the grant is still
accepted by Monarch.

## Built-in tool

```sh
codetether mcp call --tool monarch --arguments '{"action":"list_tools"}'
```

The model-facing operations are:

```json
{"action":"status"}
{"action":"list_tools"}
{"action":"call","tool":"NAME_FROM_CATALOG","arguments":{}}
```

Use tool names and schemas from the official live catalog, not a third-party
bridge. The coding profile retains this connector, and compact discovery profiles
can discover it normally. Native CLI, TUI, worker, and MCP-server registries use
the same connector. The backend and account grant still govern access.

## Credentials and account boundaries

OAuth credentials are stored in the `monarch` provider record under its
`monarch_oauth` field through `codetether-secrets`, using the configured Vault
mount/path. Other fields are preserved. There is no separate plaintext/DPAPI
Monarch credential store; environment tokens are not accepted, and credentials
from the earlier development implementation are not imported automatically.

**Capability availability is not authorization.** Instances with access to the
same configured provider record share that Monarch account grant. Configure
separate backend paths and policies for unrelated users or tenants. This change
does not broaden those policies or propagate credentials through the agent bus.
Do not authorize a personal account into a backend readable by unrelated users.

A host-local file lock serializes authentication and refresh on that host; the
lock contains no credentials. Refreshed credentials are saved before use, with a
stale-record check. This is **not distributed compare-and-swap**: independent
hosts sharing one rotating grant must not refresh concurrently. Deployment-wide
refresh coordination is not provided by this initial integration.

OAuth uses PKCE S256, state validation, and the pinned official resource
`https://api.monarch.com/mcp`. The CLI requests only `mcp:read`. Tool execution
rejects write-scoped credentials, unannotated tools, and tools without
`annotations.readOnlyHint=true`. HTTPS requests are bounded and do not follow
redirects carrying credentials. Tool failures are not automatically replayed.

Monarch controls supported accounts and usage limits. This connector does not
bypass unavailable institutions or rate limits. Returned financial data remains
external tool content, not instructions trusted by the agent runtime.

## Disconnect

```sh
codetether auth monarch logout
```

Logout attempts revocation, then removes the OAuth field from the configured
provider record even if revocation fails. Backend write failures remain errors.
A revocation failure requires disconnecting CodeTether in Monarch's settings as
well. Vault version history and backups may retain earlier records; removing
the current field is not forensic erasure.

The legacy `family-budget` scripts and sessions are not imported, changed, or
deleted. Retire separate jobs explicitly when migrating them.

## Focused development checks

```sh
cargo test -p codetether-monarch-auth -p codetether-monarch --all-targets
cargo test -p codetether-monarch-auth -p codetether-monarch --doc
cargo test -p codetether-secrets strict_provider_reads --lib
cargo test -p codetether-auth-cli monarch_auth --lib
cargo clippy -p codetether-monarch-auth -p codetether-monarch --all-targets --no-deps -- -D warnings
```

Fixture tests establish local behavior, not live account access. Live evidence
requires browser consent and a successful authenticated official MCP catalog
request. Historical local-store evidence is retained under
`artifacts/monarch/superseded-local-store/` and does not verify this backend.
