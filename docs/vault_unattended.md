# Unattended Vault authentication

`--yolo` must not introduce an extra approval prompt for credentials already
delegated to CodeTether. Credential recovery runs in the runtime, not through
model-visible token values. It also works in other modes, like existing renewal.
This does not bypass Vault policy, IdP consent, or service account permissions.

## One-time provisioning

An authorized Vault operator must provision a dedicated AppRole with only the
required provider paths. Include self-lookup/self-renew permissions. For Monarch,
the `monarch` provider record needs read and update permissions for OAuth rotation;
initial creation also needs create. Do not grant root or general token creation.

Supply its RoleID and an owner-private SecretID file on the runtime host. Restrict
the file with Unix mode 0600 or a Windows ACL limited to the service/user account.
Use your existing secure provisioning channel, not chat, command arguments,
repository files, or logs. This feature does not provision or expand Vault policy.

## Runtime configuration

Set these process environment variables (PowerShell can set the same names):

```sh
export VAULT_ADDR='https://vault.example.com'
export VAULT_AUTH_METHOD=approle
export VAULT_AUTH_MOUNT=approle
export VAULT_ROLE_ID='YOUR_PROVISIONED_ROLE_ID'
export VAULT_SECRET_ID_FILE='/absolute/private/path/vault-secret-id'
codetether --yolo
```

Explicit AppRole selection takes precedence over saved desktop tokens and
`VAULT_TOKEN`. Failed login does not silently select another identity. Optional
`VAULT_MOUNT` and `VAULT_SECRETS_PATH` select the existing KV provider namespace.
No browser or per-token approval is used. SecretIDs are re-read at login, so an
external provisioner can rotate the file without restarting CodeTether.

Renewable tokens retain normal lease renewal. On a rejected secret operation,
the runtime logs in again and retries once; clones coordinate the replacement.
If renewal stops while idle, reauthentication happens on the next rejected
operation, not in an independent background login service. Finite-use/expired
SecretIDs need external replacement; `--yolo` cannot resurrect a revoked identity.

This updates CodeTether's in-process client, not the shell's `VAULT_TOKEN` or the
standalone Vault CLI token cache. Shell `vault` commands may still use a different
credential. These changes require an updated CodeTether binary and restart; they
do not upgrade an already-running agent or resolve Monarch's server-side 403.