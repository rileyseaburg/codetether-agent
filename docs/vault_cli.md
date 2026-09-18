# First-class `codetether vault` commands

These commands are implemented in source and require a build containing this change. The already-published `v4.7.6-dev.6` binaries do not contain them. No model is started for Vault management.

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

## Device-code login

```sh
codetether vault login device --issuer https://auth.quantum-forge.io/realms/spotlessbinco.com --client-id codetether-cli --mount oidc --role codetether-device --no-browser
```

The client ID and role above are example deployment configuration, not credentials or automatically provisioned resources. The IdP needs a **public** device-enabled client. Vault needs a **JWT-type** role trusting that issuer/audience and restricting the approved CodeTether group. CodeTether polls the IdP, exchanges its JWT for a Vault token, then validates it. This is not a fictional Vault device-authorization endpoint. Missing server/client configuration is an error, never an admin fallback.

## Start work or remove the local login

```sh
codetether models
codetether vault logout
```

Authentication does not grant provider permissions; the Vault role must supply those separately. Logout removes the saved local credential without revoking someone else's token.

Saved settings are per-user, outside the checkout: owner-private files on Unix, and current-user DPAPI protection on Windows. They take precedence over stale inherited Vault variables for subsequent CodeTether launches. `CODETETHER_VAULT_SOURCE=env` explicitly bypasses this profile; Kubernetes `VAULT_ROLE` authentication retains its existing precedence. Existing processes and parent-shell variables are not modified. `CODETETHER_VAULT_CONFIG_DIR` can isolate the storage directory for controlled automation.