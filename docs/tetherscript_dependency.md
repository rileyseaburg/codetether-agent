# TetherScript Dependency

CodeTether uses TetherScript's scripting VM for the `tetherscript_plugin` tool
and, since v0.1.0-alpha.12, as a deterministic browser substrate behind
`codetether browserctl offline ...` (see `docs/browserctl_offline.md`).

The crate is published on crates.io as `tetherscript`. v0.1.0-alpha.12 adds
`browser_session::BrowserSession` (in-process deterministic browser),
`browser_cookie::Cookie` (Set-Cookie parsing/jar), and a real
`export_trace_json` path on `browser_cap::invoke` for action/auth traces.

## Version

- **Crate**: `tetherscript` v0.1.0-alpha.12
- **Source**: <https://crates.io/crates/tetherscript>
- **Repository**: <https://github.com/CodeTether/TetherScript>

## Capabilities available

| Capability module | Authority type | Purpose |
|---|---|---|
| `tetherscript::plugin::TetherScriptAuthority` | Base scripting builtins |
| `tetherscript::provider_cap::ProviderAuthority` | HTTP provider calls |
| `tetherscript::browser_cap::BrowserAuthority` | Live browser control via CDP bridge |
| `tetherscript::fs_cap::FsAuthority` | Filesystem access |
| `tetherscript::rpc_cap::RpcAuthority` | RPC calls |

The version is locked in both `Cargo.toml` and `Cargo.lock` to keep builds
reproducible.
