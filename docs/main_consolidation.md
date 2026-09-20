# Main-branch consolidation

The September 20 consolidation keeps implementation in the normal repository
paths on `main`, not in secondary worktrees or artifact directories.

## Integrated work

- Pending approval-review, LSP ownership/cache, native HTTPS, installer, and
  TetherScript source from the primary checkout.
- Process-tree cancellation, including Windows Job Objects and platform tests.
- Local model catalog, quantized-CUDA prefill chunking, and voice transport work.
- Mux clipboard/paste handling, Mermaid rendering, worktree recovery, and
  bounded startup discovery.

Older Vault, Windows desktop, sandbox, and mux variants were reconciled with
the newer implementations already on `main`; they were not copied over newer
security checks or explicit-revision isolation rules. Generated build output
and credential-bearing cookie captures were not added as product source.

## Preserved records

Historical Git refs and worktree/evidence snapshots are retained in
private files outside this checkout under:

`~/.local/state/codetether-main-consolidation-20260920T184346Z.*`

The `.sha256` receipt records archive identities. These files are recovery and
validation records only, not another working checkout. Do not publish them:
historical captures may contain private runtime data.

## Validation boundary

Static/local formatting, whitespace, conflict-marker and file-limit checks
cover the integrated source. Compilation, platform tests and full runtime
regression coverage were not run as part of this consolidation.