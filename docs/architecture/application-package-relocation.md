# Application package relocation

The application package now lives in `crates/codetether-agent`; the repository
root is a virtual workspace. Its source, integration tests and Rust examples moved
together. Component implementations retain their dedicated owning crates.
The remaining work is the final ownership and stale-reference audit.

The package-level change moved `codetether-agent` into
`crates/codetether-agent`. It preserves its library name `codetether_agent`, binary name
`codetether`, features and dependency versions unchanged. This is the application
composition crate; domain implementations retain their existing owning crates.

## Relocation checklist

1. Split the existing root manifest. Keep workspace membership, profiles and
   registry patches at the repository root. Move package, library, binary,
   feature and dependency tables into the application crate. Replace the `.`
   workspace member and set the application as the default member. Rebase its
   local dependency paths without changing dependency identities or versions.
2. Move `src/` and the application's integration tests with their relative
   module structure intact. Preserve source contents except explicit path fixes.
   Keep uncommitted user changes uncommitted and preserve their contents when
   their source paths move.
3. Preserve the three Rust example targets (`browser_validate`, `oracle_demo`,
   `thinker_example`) while leaving runtime TetherScript examples at their
   documented repository paths.
4. Update repository-root assumptions in
   `src/session/store/tests/incident.rs`, `tests/fixtures/workspace.rs`, and
   `tests/worktree_module_graph.rs`. Relative test imports into `../src/`
   should remain valid when tests move alongside the application source.
5. Update Docker's `COPY src ./src`, release-version readers, development
   version scripts, install paths, and source-audit roots. Relevant readers
   include `release.sh`, `script/{bump,set}-dev-version.sh`, and the release
   metadata/macOS workflows. Preserve their existing version-update semantics.
6. Verify every moved source against its prior content; inspect workspace
   membership, dependency targets, target discovery, include paths, and all
   live scripts that address the old package location. Directory relocation
   alone does not prove completion of the broader cohesion goal.

## Static-check prerequisites implemented

`script/architecture/application_layout.mjs` identifies exactly one application
package at either supported location and rejects ambiguous or missing layouts.
The declaration audit uses its source root. The file-size ratchet checks both
locations and carries a moved file's original baseline forward only when the
old source path no longer exists. Copies and new files retain the new-file limit.
Explicit Git glob pathspecs also cover files directly beneath `src/`.

Five Node tests exercise current/relocated layouts, ambiguity, wrong package
identity, unchanged oversized moves, oversized growth, and copied/new-file
limits. These tests execute only Node, Bash, Git and native filesystem operations.
They do not compile Rust or prove Rust type correctness.

Cargo, rustc and Python remain prohibited. The full consolidation goal remains
active until the relocation and remaining ownership audit are actually complete.

## Validation status

The relocation verifier accounts for 1,281 moved source/test files and three
Rust examples. Of the source/test files, 1,273 retain identical bytes; eight have
explicit path or stale-import fixes. Source hashes preserve the two preexisting
uncommitted edits at their relocated paths, and their index entries retain the
original committed content. Cargo.lock is unchanged.

Five Node layout/line-budget tests and the shell version-bump regression pass.
The latter now tests both root-package and virtual-workspace layouts. Module and
dependency scans pass with 618 packages. No Cargo, rustc, Python, Rust tests or
Docker builds ran. Directory relocation is complete; the full consolidation goal
still requires its final audit.
