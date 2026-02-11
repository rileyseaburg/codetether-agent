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

### Overrides

- `CODETETHER_TAG` / `CODETETHER_VERSION`: force a specific release tag/version (for example `v1.2.3`).
- `CODETETHER_GITHUB_REPO`: override the repo (default: `rileyseaburg/codetether-agent`).

## License

MIT
