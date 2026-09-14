# Linux and macOS: copy-and-paste installation

Use Bash or zsh for these examples. Published targets are Linux x86-64, Apple Silicon macOS, and Intel macOS. A detected architecture alone does not imply a release asset exists for it.

## 1. Install or update

Run as your normal user. The installer requests sudo only if needed for its chosen destination.

```sh
curl -fsSL https://raw.githubusercontent.com/rileyseaburg/codetether-agent/main/install.sh | sh
```

Wait for a successful install message. Downloads come from GitHub and must match the release checksum manifest. Stop on checksum, extraction or permission errors.

## 2. Verify the executable you will run

```sh
command -v codetether
codetether --version
```

Compare with [GitHub releases](https://github.com/rileyseaburg/codetether-agent/releases), currently `4.7.6-dev.6`. If missing or older, do not assume another terminal fixes it. Check the exact destination printed by the installer:

```sh
if [ -x /usr/local/bin/codetether ]; then /usr/local/bin/codetether --version; fi
if [ -x "$HOME/.local/bin/codetether" ]; then "$HOME/.local/bin/codetether" --version; fi
if [ -x "$HOME/.cargo/bin/codetether" ]; then "$HOME/.cargo/bin/codetether" --version; fi
```

Use the explicit path that reports the expected version for subsequent commands. If setup chose `~/.local/bin`, this updates only the current shell's PATH:

```sh
export PATH="$HOME/.local/bin:$PATH"
hash -r
command -v codetether
codetether --version
```

Do not delete another installation blindly. macOS artifacts are unsigned; this guide does not promise notarization or advise disabling Gatekeeper. If macOS rejects execution, retain its exact diagnostic.

## 3. Configure credentials and launch

Follow [Unix Vault setup](install_unix_vault.md). Optional local routing-model download: append `-s -- --functiongemma` after `sh` in step 1. This is not required to install or run CodeTether.