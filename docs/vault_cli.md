# First-class `codetether vault` commands

These commands are implemented in source and require a build containing this change. The already-published `v4.7.6-dev.6` binaries do not contain them. No model is started for Vault management.

## Quick workflow: existing Vault token

Run these commands in PowerShell, bash, or zsh:

```text
codetether vault url https://vault.spotlessbinco.com
codetether vault login token
codetether vault status
codetether models
```

Use your own server URL if different. Enter the token at the login prompt.
Check for `authenticated: true` from status, then a nonempty model list.

**`-t` / `--token` authenticates the A2A/MCP control plane, not Vault.**
Changing that argument does not replace the saved Vault login or `VAULT_TOKEN`.
The same workflow is available in `codetether --help`, `codetether vault --help`,
and `codetether vault login --help`.

If status reports `403 invalid token`, run `codetether vault login token` again
with the intended credential. A saved login normally takes precedence over
`VAULT_TOKEN`; `CODETETHER_VAULT_SOURCE=env` bypasses it, and `VAULT_ROLE` selects
workload authentication.

If models reports `configured; no models discovered`, registration alone has
not established usable model access. Check Vault provider permissions, or the
AWS profile and region when intentionally using local AWS fallback.

For an editor extension connected to a CodeTether server, configure Vault on
the **server host, under the account running that server**. A local CLI login
does not update a remote server or an already-running process. The extension's
server-authentication token is separate from the server's Vault credential.

## Configure and inspect

```sh
codetether vault url https://vault.spotlessbinco.com
codetether vault status
```

Changing the URL discards the saved credential for the previous server. Status never displays a token. URL, status, and login work independently of provider startup.

## Browser OIDC

```sh
codetether vault login oidc --mount oidc --role codetether
```

This uses Vault CLI's browser/callback implementation; `vault` must be installed on PATH. No token is printed or stored in Vault CLI's cache. For SSH, use `--no-browser` and forward the browser's `localhost:8250` to the VM's callback listener. The operator must provision the named app-scoped role; CodeTether never selects `admin` automatically.

## Existing Vault token

```sh
codetether vault login token
```

The prompt hides input. Automation can use `codetether vault login token --stdin` with a secure pipe. Tokens are not accepted as command-line arguments. Authentication and administrator-capability checks run before the previous saved login is replaced.

## Device-code login (recommended)

```sh
codetether vault login device
```

No Vault CLI and no pasted token. CodeTether prints a short code and opens the
sign-in page (`--no-browser` prints the link instead, for SSH sessions or another
device). Approve it as a member of the `vault-admins` Keycloak group; CodeTether
exchanges the result for a read-only Vault token and saves it.

Defaults: issuer `https://auth.quantum-forge.io/realms/spotlessbinco.com`, public
client `codetether-cli`, Vault JWT mount `jwt`, role `codetether-device`. Override
with `--issuer/--client-id/--mount/--role` or `CODETETHER_VAULT_DEVICE_ISSUER`,
`CODETETHER_VAULT_DEVICE_CLIENT_ID`, `CODETETHER_VAULT_DEVICE_MOUNT`,
`CODETETHER_VAULT_DEVICE_ROLE`. The Keycloak client, audience mapper and Vault role
definitions live in `script/keycloak/`. Rejections now include Vault's reason
(e.g. an audience or group mismatch) when it contains no credential.

## Start work or remove the local login

```sh
codetether models
codetether vault logout
```

Authentication does not grant provider permissions; the Vault role must supply those separately. Logout removes the saved local credential without revoking someone else's token.

Saved settings are per-user, outside the checkout: owner-private files on Unix, and current-user DPAPI protection on Windows. They take precedence over stale inherited Vault variables for subsequent CodeTether launches. `CODETETHER_VAULT_SOURCE=env` explicitly bypasses this profile; Kubernetes `VAULT_ROLE` authentication retains its existing precedence. Existing processes and parent-shell variables are not modified. `CODETETHER_VAULT_CONFIG_DIR` can isolate the storage directory for controlled automation.