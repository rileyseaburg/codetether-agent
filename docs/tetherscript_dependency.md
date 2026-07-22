# TetherScript Dependency

CodeTether uses TetherScript's scripting VM for the `tetherscript_plugin` tool,
the deterministic browser substrate behind `codetether browserctl offline ...`,
and the native default backend for `browserctl` sessions.

## Version

- **Crate**: `tetherscript` v0.1.0-alpha.23
- **Source**: <https://crates.io/crates/tetherscript>
- **Repository**: <https://github.com/CodeTether/TetherScript>

## Current language features

Alpha.23 includes closures, recursion, lexical scopes, `let mut`, `for x in
iterable`, runtime `move` ownership checks, `Result` plus `?`, byte strings,
JSON, HTTP, SMTP, filesystem/process/path/time/hash tools, and browser/JS
agent primitives.

## Native browserctl backend

With the `tetherscript` feature enabled, `browserctl start` creates an
in-process `tetherscript::browser_agent::BrowserPage` session. DOM actions,
navigation, evaluation, screenshots, tabs, network replay, diagnostics, cookies,
CORS checks, redirects, form interaction, and HAR-style evidence route through
TetherScript primitives instead of the old CDP automation stack.

## Capability modules

| Capability module | Purpose |
|---|---|
| `TetherScriptAuthority` | Base scripting builtins |
| `ProviderAuthority` | HTTP provider calls |
| `BrowserAuthority` | Browser control via native browserctl bridge |
| `FsAuthority` | Filesystem access |
| `RpcAuthority` | RPC calls |

The version is locked in both `Cargo.toml` and `Cargo.lock` to keep builds
reproducible.
