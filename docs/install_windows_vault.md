# Windows: Vault credentials and first launch

Use the same normal PowerShell in which you verified the executable. Obtain a valid token from your Vault administrator or approved login flow. Never put a token in a command, screenshot, chat, or support log. There is no `codetether vault` command in this release.

## 1. Choose ONE credential source

**A — The installer/Windows User environment holds your latest token.** Load it explicitly; merely opening a terminal tab is not proof it was inherited. Do not use this block if your newer token exists only in the current `$env:VAULT_TOKEN`.

```powershell
$savedAddress = [Environment]::GetEnvironmentVariable('VAULT_ADDR', 'User')
$savedToken = [Environment]::GetEnvironmentVariable('VAULT_TOKEN', 'User')
if ([string]::IsNullOrWhiteSpace($savedAddress) -or [string]::IsNullOrWhiteSpace($savedToken)) { throw 'Saved settings are incomplete; use option B instead.' }
$env:VAULT_ADDR = $savedAddress; $env:VAULT_TOKEN = $savedToken
Remove-Variable savedAddress, savedToken
```

**B — Enter a fresh token privately for THIS terminal.** Paste the block, then answer its prompts. This does not replace your saved User token. If the current terminal already has the correct token, skip A and B.

```powershell
$env:VAULT_ADDR = (Read-Host 'Vault HTTPS address').Trim().TrimEnd('/')
$secret = Read-Host 'Vault token (hidden)' -AsSecureString
$pointer = [Runtime.InteropServices.Marshal]::SecureStringToBSTR($secret)
try { $env:VAULT_TOKEN = [Runtime.InteropServices.Marshal]::PtrToStringBSTR($pointer) }
finally { [Runtime.InteropServices.Marshal]::ZeroFreeBSTR($pointer); $secret.Dispose(); Remove-Variable secret, pointer }
```

Only use your organization's approved Vault address. The installer defaults to KV mount `secret` and path `codetether/providers`; keep any administrator-provided `VAULT_MOUNT`/`VAULT_SECRETS_PATH` overrides. Saved User tokens are environment variables, not encrypted credential storage.

## 2. Check access before starting work

```powershell
codetether models
```

Do not start coding until your intended provider/model is listed. An accessible but empty Vault may need the provider-auth step below. A 403 `invalid token` means Vault rejected the token that process sent; saved and process tokens can differ. `permission denied` alone can be a policy issue. Reinstalling does not repair either condition. Never share `$env:VAULT_TOKEN` or a full environment dump.
If you need to add a Codex account, **after Vault access works**, paste the following and follow the browser/device instructions. This stores provider credentials in Vault; it does not log you into Vault.
```powershell
codetether auth codex
codetether models
```


## 3. Start CodeTether

```powershell
codetether tui
```

If you verified a package-family alias instead of the bare command, use `& $alias models`, `& $alias auth codex`, and `& $alias tui` in that same terminal. See [Windows alias checks](install_windows.md). Interactive Windows mux sessions need a ConPTY backend; do not treat the Linux mux instructions as Windows support.