# TetherScript Dependency

CodeTether uses TetherScript's scripting VM for the `tetherscript_plugin` tool,
the deterministic browser substrate behind `codetether browserctl offline ...`,
and the native default backend for `browserctl` sessions.

## Version

- **Crate**: `tetherscript` v0.1.0-alpha.31 (resolved in `Cargo.lock`)
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

`Cargo.toml` specifies the compatible version requirement; `Cargo.lock` pins
the resolved version for reproducible builds. Use `--locked` in CI.

## Native HTTPS requirement

The harness dependency must enable both `tera` and `openssl-tls`. TetherScript's
default features are empty: native `http_request` cannot construct a TLS
connector without `openssl-tls`. TLS features on CodeTether's other HTTP clients
do not enable TLS in the embedded scripting VM.

The `openssl-tls` feature uses vendored OpenSSL and native platform CA roots.
The client verifies certificate chains and hostnames, requires TLS 1.2 or newer,
and fails if no usable trust anchors are available. Keep the platform CA store
installed, including the approved CA for private HTTPS services. Do not bypass
verification or replace HTTPS with HTTP.

Inspect wiring without building:

```bash
cargo tree --locked --offline -e features -i tetherscript
```

Run the focused connector regression (requires the platform CA store):

```bash
cargo test --locked --test tetherscript_native_tls
```

This regression constructs the native connector in-process; it does not prove
a remote handshake or an authenticated CLI flow. Those need separate runtime
evidence. Dependency feature changes require rebuilding the **CodeTether harness
binary** through the normal CI/release process and safely restarting the process
executing `tetherscript_plugin`. Updating the standalone `tetherscript` executable,
editing a plugin, or changing the manifest cannot refresh a VM already linked
into a running harness. Preserve active sessions and coordinate the restart with
their owners.

An opt-in public HTTPS probe exercises the actual embedded plugin tool:

```bash
cargo test --locked --test tetherscript_native_https_probe -- --ignored
```

It sends only `HEAD https://example.com/`, without authentication or cookies,
and retains only the HTTP status. This tests public HTTPS, not configured Vault
trust or authentication. A new test process is not activation evidence for an
existing interactive harness; repeat the inline hook in that target after
safe activation. Never include business payloads in a TLS probe.