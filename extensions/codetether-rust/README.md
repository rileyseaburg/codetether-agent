# CodeTether Rust Development

## Independent status and actions

The **CodeTether Rust** status-bar item shows workspace task activity, the last
task outcome, and CodeTether error/warning counts. Click it, or run **CodeTether
Rust: Status and Actions**, for checks, builds, Problems, reviewed fixes, settings,
check-on-save controls, daemon actions, and cache statistics.

The rust-analyzer VS Code extension may remain completely disabled. This UI,
compiler problem matcher, and reviewed fixes do not depend on it.

## Go-to and symbols

CodeTether starts a persistent **rust-analyzer binary** for navigation in each
trusted Cargo workspace. The rust-analyzer VS Code extension is not required.
Use **F12** for definition, **Ctrl+F12** for implementation, **Shift+F12** for
references, **Ctrl+Shift+O** for document symbols/Outline, and **Ctrl+T** for
workspace symbols (standard Linux/Windows bindings). Unsaved buffer changes are
synchronized through the language client rather than read back from disk.

Install `rust-analyzer` on the extension host, or set
`codetether.rust.navigation.serverPath` to its executable. Navigation is enabled
by default; disable `.navigation.enabled` to stop it. **CodeTether Rust: Restart
Navigation Server** restarts only these clients. See the **CodeTether Rust
Navigation** output channel for startup errors. An active rust-analyzer extension
causes CodeTether to relinquish navigation to avoid duplicate providers.

These are separate clients from native/TUI LSP sessions: they share the Rust
language-server implementation, not process state. Automatic Cargo checks, build
scripts and proc macros remain disabled in these clients. Indexing and Cargo
metadata still use resources; generated/proc-macro symbols may therefore be
incomplete. This is not a claim of full rust-analyzer extension parity.

## Imports, editing assists and live issues

The same persistent language client also exposes standard rust-analyzer editing
features; no additional language-server process or AI request is needed:

- **Auto-import completion:** type a symbol and use **Ctrl+Space**. Accepting an
  out-of-scope suggestion inserts its `use` statement as well as the symbol.
  This imports available symbols; it does not add dependencies to `Cargo.toml`.
- **Quick fixes / refactor assists:** put the cursor on a name or expression and
  press **Ctrl+.**. For example, choose an import for an unresolved name. Available
  assists depend on the cursor, selected range and rust-analyzer version. Edits
  use VS Code's normal editor undo; nothing is automatically fixed on save.
- **Rename:** **F2** renames a symbol and its references via workspace edits.
- **Hover and signature help:** hover for types/docs; **Ctrl+Shift+Space** shows
  parameter information at a call.
- **Inlay hints:** inferred types and parameter/chaining hints are available;
  display also respects VS Code's `editor.inlayHints.enabled` setting.
- **Live issues:** rust-analyzer syntax/semantic diagnostics update while typing,
  including unsaved buffers, and disappear when corrected. They use the separate
  `codetether-rust-navigation` collection, with source `rust-analyzer` in Problems.
  They are not a substitute for rustc/Clippy or ordinary Cargo checking.

These workspace-scoped settings default to `true` and apply without restarting
the server:

```json
{
  "codetether.rust.navigation.autoImport": true,
  "codetether.rust.navigation.diagnostics": true,
  "codetether.rust.navigation.inlayHints": true
}
```

Cargo task diagnostics and shared **CodeTether LSP** diagnostics retain their own
collections. Disable `.navigation.diagnostics` if the shared feed produces
duplicate issues; doing so clears only the editor client's markers, not the other
collections. The separate **Fix with CodeTether** action still requires source
sharing consent, saved snapshots and reviewed application. Built-in import assists
and other rust-analyzer edits do not invoke that AI repair workflow.

## sccache / MinIO

Choose **Use sccache (existing MinIO/S3 configuration)…** in the status menu.
After confirmation, Cargo tasks receive `RUSTC_WRAPPER=sccache` and
`CARGO_INCREMENTAL=0`; `.sccacheExecutable` selects a literal host executable.
Existing host-side cache configuration and credentials are inherited, not stored
in VS Code settings. No daemon is restarted or credentials changed. Existing
ra-jit daemons retain their environment; their in-memory reuse is separate.

**Cache Statistics** shows only backend type and aggregate counts. S3-compatible
configuration alone does not establish MinIO connectivity or cache hits. Counts
are daemon-wide, not evidence of reuse by this workspace.

## Shared release cache for `make be`

Use **Build & Publish Shared Cache** in the status menu or command palette to
run `codetether rust --cwd <workspace> cache-build`. It invokes the repository's
`bash scripts/dev/release-build-loop.sh build`, then `publish` only if the build
succeeds. **Publish Verified Cache** (`cache-publish`) retries publication without
rebuilding, for example after committing the exact sources already built.

These explicit, confirmed actions require that repository-owned script and a
saved trusted Cargo workspace. They are never run on save. The repository owns
the release targets, compiler selection, shared sccache configuration, memory
gate, locking, fingerprints, smoke checks, receipts, and cluster-ci-cache upload
verification. Editor package/bin/toolchain/sccache overrides are not applied to
these actions. Host credentials remain outside extension settings and logs.

A dirty source tree can defer publication with exit zero; the status bar therefore
says to inspect the publication log, not that an upload succeeded. A later
`make be` can reuse matching verified artifacts under its existing restore rules;
neither an exit-zero task nor wrapper configuration proves a remote cache hit.
ra-jit build/run/test and ordinary check-on-save remain separate, and their outputs
are not promoted to release artifacts. No backend is started or restarted.
Requires a backend whose `codetether rust --help` lists `cache-build` and
`cache-publish`; older backends reject these actions.

Explicit ra-jit build/run/test commands backed by the first-party `codetether rust`
CLI, plus an ordinary Cargo check command. This extension keeps rust-analyzer
unchanged; it does not accelerate autocomplete, indexing, or borrow checking.

## Setup

Install a CodeTether version that includes `codetether rust`, ra-jit, and Rust
1.99.0 with rust-src on the extension host. Use **CodeTether Rust: Diagnose
Prerequisites** to check availability. No installation or global enable happens
automatically. Remote windows execute on the remote host.

Open a trusted Cargo workspace and use **CodeTether Rust: Build (ra-jit)**.
Configure `codetether.rust.package`, `.bin`, `.toolchain`, and literal `.runArgs`
to select your target. `.executable` is the machine-scoped CodeTether path.
Tasks provide output, Rust problem matching, and VS Code task cancellation.

The initial build may use Cargo, and unsupported edits may fall back to it.
ra-jit has no borrow checker: retain **CodeTether Rust: Check (rustc)** and CI.
Stopping the shared daemon requires confirmation. Agent calls use the same CLI
through `rust_build_plan` followed by ordinary approval-aware `exec_command`.

## Check on save

Run **CodeTether Rust: Enable Cargo Check on Save** once per workspace, or set
`codetether.rust.checkOnSave` to true. Rust/Cargo saves are debounced (750ms by
default, configurable with `.checkOnSaveDelay`), and checks are serialized per
workspace. Checks use ordinary Cargo, not ra-jit. They update **CodeTether Rust**
Problems without automatic edits. The enable command offers to disable
rust-analyzer's duplicate check-on-save; it does not disable its language server.

## Shared TUI diagnostics

New CodeTether native/TUI processes publish their existing LSP diagnostics; this
extension displays them as **CodeTether LSP** for the matching canonical workspace.
No additional LSP is launched. Empty publications, exited producers, and stale
heartbeats clear their markers. Both processes must run as the same user on the
same host. `.sharedDiagnostics` defaults true. `.diagnosticsDirectory` can match
the native `CODETETHER_DIAGNOSTICS_DIR` override. Existing TUI processes need a
restart onto the new binary to publish; extension installation cannot retrofit them.

## Reviewed diagnostic fixes

Select **Fix with CodeTether** from a Rust diagnostic's lightbulb. After consent,
the native `codetether rust-propose` command sends bounded source/manifest and
applicable instructions snapshots to the configured provider (`.fixModel` can
override it). It has no tools or project-write authority. Review each diff, then
choose **Apply & Check** to apply and run Cargo check for the owning package.
Dirty/stale snapshots, edits outside the captured scope, and ambiguous replacements
are rejected. This requires a backend with `rust-propose`; it never loads sessions.

Tests: `npm test`; on Linux, `xvfb-run -a npm run test:vscode` runs the isolated editor fixture.
Run `xvfb-run -a npm run test:vscode -- --editing` for the focused real-rust-analyzer
editing suite (imports, live issues, rename, hover, signatures, hints and settings).
Evidence and exit status are retained under `artifacts/rust-problems-vscode-*`.