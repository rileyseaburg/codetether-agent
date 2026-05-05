# TetherScript Dependency

CodeTether uses TetherScript's scripting VM for the `tetherscript_plugin` tool.

The crate is published on crates.io as `tetherscript`. As of v0.1.0-alpha.10 it
includes a live browser capability (`browser_cap::BrowserAuthority`) that posts
JSON commands to a CodeTether/browserctl-compatible HTTP bridge.

## Version

- **Crate**: `tetherscript` v0.1.0-alpha.10
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
