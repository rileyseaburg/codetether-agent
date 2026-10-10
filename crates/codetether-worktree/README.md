# CodeTether worktree lifecycle

`codetether-worktree` owns creation, discovery, integration, cleanup, and editor
integration for managed Git worktrees. `WorktreeManager` is the entry point;
`maintenance` exposes cleanup planning and reports. The agent preserves its
existing API by re-exporting the crate from `codetether_agent::worktree`.

## Dependency boundary

- `codetether-provenance`: repository discovery and attributed integration commits.
- `codetether-worktree-isolation`: Cargo workspace stubs in managed checkouts.
- No dependency on the agent crate, providers, tools, or the TUI. The UI-active
  flag remains in this crate so callers and editor-prompt suppression share it.
- `WorktreeManager::repo_path()` exposes the repository read-only for swarm branch
  checks without making the manager's fields public.

Existing lifecycle tests live beside the extracted implementation. Test fixtures
create disposable repositories; no application credentials are needed.
Editor tests share a guard that serializes changes to process-wide environment
variables and the TUI-active flag, restoring their original values on exit.

## Focused checks

```sh
cargo test -p codetether-worktree --lib --tests
cargo test -p codetether-worktree --doc
cargo clippy -p codetether-worktree --all-targets -- -D warnings
```

The integration tests compile the agent facade and verify shared type identities.

## Build-performance scope

This separation provides an independent compilation and test unit. No before/after
build benchmark has been run, so it does not establish a measured speedup. The
agent's release profile still uses thin LTO and one codegen unit; final application
linking can still dominate installation time.