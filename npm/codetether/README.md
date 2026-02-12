# codetether (npx)

This package wraps the **CodeTether Agent** Rust binary (`codetether`) so you can run it via **npx** without installing Rust.

## Usage

```bash
npx codetether --help
npx codetether tui
npx codetether run "explain this codebase"
```

## How it works

- On install (and again on first run if needed), it downloads the matching `codetether` binary from GitHub Releases:
  `https://github.com/rileyseaburg/codetether-agent/releases`
- It picks the correct asset for your OS/CPU.
- It caches the binary in a per-user cache directory (so it still works when `node_modules/` is read-only).

Default cache locations:

- Linux/macOS: `$XDG_CACHE_HOME/codetether-npx` or `~/.cache/codetether-npx`
- Windows: `%LOCALAPPDATA%\codetether-npx`

### Overrides

- `CODETETHER_TAG` / `CODETETHER_VERSION`: force a specific release tag/version (for example `v1.2.3`).
- `CODETETHER_GITHUB_REPO`: override the repo (default: `rileyseaburg/codetether-agent`).
- `CODETETHER_NPX_CACHE_DIR`: override where the binary is cached.
- `CODETETHER_NPX_SKIP_DOWNLOAD=1`: never download; only run if already cached.
- `CODETETHER_NPX_NO_CHECKSUM=1`: skip SHA-256 verification (not recommended).

## FunctionGemma model

This npm wrapper only installs/runs the `codetether` binary. If you also want the FunctionGemma model (~292MB) and shell auto-config, use the official install scripts in the main repo README.

## License

MIT
