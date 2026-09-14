# Windows: replace a Vault token without reinstalling

Open PowerShell **normally**, not as administrator, and paste:

```powershell
irm https://forgejo.quantum-forge.io/riley/codetether-agent/raw/branch/main/update-vault.ps1 | iex
```

1. Confirm or change the Vault address. Check the displayed server before entering a token.
2. Enter a replacement token at the **hidden prompt**, never in a command or chat.
3. Wait for the success message. The updater checks token renewal and configured provider access **before** saving it.
4. Verify access from the same PowerShell:

```powershell
codetether models
```

## What changes

- Saves `VAULT_ADDR` and `VAULT_TOKEN` in your Windows **User** environment, not Machine scope.
- Uses a normal child Windows PowerShell for the private prompt, then explicitly reloads the saved settings into the calling PowerShell after success.
- Existing CodeTether processes and other terminals keep their old environment. A new tab in an old terminal host is not proof of refresh.
- Does not install CodeTether, register MSIX, change its alias, install OCR, or request UAC. An MSIX registration failure does not block this credential-only updater.
- Tokens remain environment variables, not encrypted credential storage. No token is written to a transcript or passed as a process argument.

## Requirements and failure behavior

- Use your approved HTTPS Vault address. Plain HTTP is allowed only for loopback development; credential-bearing requests do not follow redirects.
- The token must be renewable (or verifiably non-expiring), not use-limited, and able to list the configured provider path and read at least one existing provider entry. Defaults: mount `secret`, path `codetether/providers`.
- Rejected validation does not overwrite the old token. Obtain the correct token/policy rather than repeating installation.
- Helpers are fetched from a pinned commit and Git-blob hash checked before execution. They remain under `%LOCALAPPDATA%\codetether\vault-recovery`.
- Stronger organization/signing policies are preserved. The child uses process-local `RemoteSigned`; only those verified helper files are unblocked. If policy refuses this, request an approved signed updater—do not weaken global policy.
- For first-time empty Vaults or manually managed process credentials, see [manual setup](install_windows_vault.md).