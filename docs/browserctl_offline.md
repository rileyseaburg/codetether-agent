# `browserctl offline` — TetherScript-backed browser probes

Net-new capability surface introduced with the `tetherscript` v0.1.0-alpha.12
bump. These subcommands do **not** drive Chromium via DevTools — they probe
HTTP/cookie/CORS state directly and replay captures through the deterministic
`tetherscript::browser_session::BrowserSession`.

Use them when launching Chromium is overkill or actively harmful: CI smoke
tests, golden auth-flow snapshots, offline reproduction of a captured page.

## Commands

| Command | Purpose |
|---|---|
| `auth-trace <url>` | Walk the redirect chain, capture `Set-Cookie` per hop, emit a JSON trace |
| `cookie-diff <before.json> <after.json>` | Diff two JSON cookie jars: added / removed / changed |
| `explain-cors <url> --origin <o> [--method <m>]` | OPTIONS preflight + classify Allow-Origin/Allow-Methods |
| `record <url> --out <file>` | Capture an HTTP GET (status + headers + body) to JSON |
| `replay <capture>` | Load the capture into a deterministic `BrowserSession` |

## Examples

```bash
codetether browserctl offline auth-trace https://app.example.com/login \
  --max-redirects 5

codetether browserctl offline cookie-diff baseline.json after-login.json

codetether browserctl offline explain-cors https://api.example.com/data \
  --origin https://app.example.com --method POST

codetether browserctl offline record https://example.com --out capture.json
codetether browserctl offline replay capture.json
```

## Scope

These probes use `reqwest` for network I/O and `tetherscript::browser_session`
for replay. They are not a Chromium replacement — JS hydration, service
workers, screenshots, and canvas/WebGL still require the existing
`browserctl snapshot|eval|screenshot` path.
