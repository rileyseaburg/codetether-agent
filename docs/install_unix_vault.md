# Linux/macOS: Vault login and manual fallback

Prefer [OIDC login through the `spotlessbinco.com` realm](vault_oidc.md). The installer offers it and prints an explicit parent-shell activation command. The steps below are a **manual-token fallback**, not the default login flow. Use the verified executable path; never paste tokens into commands, history, screenshots, or chat.

## 1. Open Bash for the following examples

```sh
bash
```

Wait for its prompt before pasting the next block. Keep this shell open for CodeTether; changing its environment does not update its parent shell. If your current shell already has verified Vault settings, skip step 2.

## 2. Enter credentials privately

```bash
read -r -p 'Vault HTTPS address: ' VAULT_ADDR
read -r -s -p 'Vault token (hidden): ' VAULT_TOKEN
printf '\n'
export VAULT_ADDR VAULT_TOKEN
```

Use your organization's approved address. These settings last only for this shell and its children; they are not saved to a profile. Keep any administrator-provided `VAULT_MOUNT`/`VAULT_SECRETS_PATH` settings; defaults are `secret` and `codetether/providers`.

## 3. Check providers before starting work

```bash
codetether models
```

Do not start coding until the intended provider/model appears. An accessible but empty Vault may need provider authentication. If Vault rejects access, fix the token/address or policy rather than reinstalling. There is no `codetether vault` command. To add a Codex account, **after Vault access works**, paste:

```bash
codetether auth codex
codetether models
```
Follow the browser/device instructions. This adds provider credentials; it is not a Vault login.

## 4. Start CodeTether

```bash
codetether tui
```

If `command -v codetether` points to an older executable, replace `codetether` in these commands with the verified absolute path, for example `"$HOME/.local/bin/codetether"`. Do not share a full environment dump when reporting an error.