# Vault OIDC login

The managed deployment uses Vault `https://vault.spotlessbinco.com`, OIDC mount `oidc`, Keycloak realm `spotlessbinco.com`, and client `vault`. Vault owns the client secret; installers never receive it.

## Desktop login (Bash or zsh)

Install the [official Vault CLI](https://developer.hashicorp.com/vault/install), then run:

```sh
export VAULT_ADDR='https://vault.spotlessbinco.com'
env -u VAULT_TOKEN vault login -method=oidc -path=oidc -no-print role="${CODETETHER_VAULT_OIDC_ROLE:-default}"
```

The browser opens the configured Keycloak realm. `-no-print` prevents the resulting Vault token appearing in the terminal; Vault saves it using its token helper. An old process token is excluded from this login.
Only after successful login, load the saved token into **this shell** without printing it:

```sh
set +x
export VAULT_TOKEN="$(env -u VAULT_TOKEN vault print token)"
codetether models
```

Do not start coding until the intended provider/model appears. OIDC authentication is not provider authorization. The current `default` role grants only Vault's default policy. `admin` is an existing privileged role restricted to `vault-admins`; installers never select it automatically. An administrator must configure an approved group/role for ordinary CodeTether access.

## Installer activation and SSH

The Unix installer offers OIDC when authentication is absent/rejected, then prints a source command for `~/.config/codetether/vault-env.sh` (or your XDG config directory). Paste that command into the parent shell. It loads a valid cached token without storing one in `.zshrc`/`.bashrc`; starting the installer via a pipe cannot export into its parent.
Under SSH, the browser's `localhost:8250` must reach the Vault CLI listener on the remote host. Establish appropriate local port forwarding before login, or use a browser on that host. For a multi-hop Windows → Ubuntu → Mac connection, forwarding only the Ubuntu → Mac hop is insufficient for a browser on Windows. Use `skip_browser=true` with `vault login` to open its displayed URL yourself; never expose the callback listener publicly.

## Windows PowerShell with Vault CLI

```powershell
$env:VAULT_ADDR = 'https://vault.spotlessbinco.com'
Remove-Item Env:VAULT_TOKEN -ErrorAction SilentlyContinue
vault login -method=oidc -path=oidc -no-print role=default
if ($LASTEXITCODE -ne 0) { throw 'OIDC login failed; stop here.' }
$cached = vault print token
if ($LASTEXITCODE -ne 0 -or -not $cached) { throw 'No cached Vault token.' }
$env:VAULT_TOKEN = $cached.Trim()
Remove-Variable cached
codetether models
```