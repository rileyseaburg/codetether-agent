# TetherScript Dependency

CodeTether uses TetherScript's scripting VM for the `tetherscript_plugin` tool,
the deterministic browser substrate behind `codetether browserctl offline ...`,
and the native default backend for `browserctl` sessions.

The crate is published on crates.io as `tetherscript`. v0.1.0-alpha.14 keeps
the browserctl bridge API compatible while adding native browser-agent release
coverage, CLI browser grants, production debug reports, and stricter package
metadata. v0.1.0-alpha.12 introduced the deterministic browser substrate used
by offline browserctl probes.

## Version

- **Crate**: `tetherscript` v0.1.0-alpha.14
- **Source**: <https://crates.io/crates/tetherscript>
- **Repository**: <https://github.com/tetherscript-Rs/tetherscript>

## Capabilities available

| Capability module | Authority type | Purpose |
|---|---|---|
| `tetherscript::plugin::TetherScriptAuthority` | Base scripting builtins |
| `tetherscript::provider_cap::ProviderAuthority` | HTTP provider calls |
| `tetherscript::browser_cap::BrowserAuthority` | Browser control via native browserctl bridge |
| `tetherscript::fs_cap::FsAuthority` | Filesystem access |
| `tetherscript::rpc_cap::RpcAuthority` | RPC calls |

## Native browserctl backend

With the `tetherscript` feature enabled, `browserctl start` creates an
in-process `tetherscript::browser_agent::BrowserPage` session.
DOM actions, navigation, evaluation, screenshots, tabs, network replay, and
diagnostics route through TetherScript primitives. The crate no longer depends
on the old CDP automation stack.

The version is locked in both `Cargo.toml` and `Cargo.lock` to keep builds
reproducible.
