# Worktree Lifecycle for Multi-Edit Agent Tasks

> Issue #297 Part B — worktree churn destroying in-flight edits.

## Problem

During multi-step agent tasks, per-turn worktree isolation could tear down
the working directory mid-task, silently discarding uncommitted edits.

## Lifecycle Contract

1. **Workspace-relative storage.** Managed worktrees are created only below
   `<workspace-root>/.codetether-worktrees/`. Temporary directories, sibling
   directories, arbitrary custom roots, and symlink redirects are rejected.
   Agent shell tools reject direct `git worktree add`; callers use the managed
   worktree layer instead.

2. **One task = one worktree.** A single agent task operates against a stable
   worktree for its full duration. The worktree must not be torn down between
   turns or steps of the same task.

3. **Dirty-check guard.** Before `git worktree remove --force`, the
   `WorktreeManager` checks `git status --porcelain`. If uncommitted changes
   exist, removal is **refused** and an error is logged:
   ```
   Refusing to force-remove dirty worktree — commit or stash first.
   ```

4. **Cleanup responsibility.** The caller (CLI command, TUI, or agent loop)
   is responsible for committing or stashing before requesting worktree
   cleanup. The worktree manager will not silently destroy work.

5. **No filesystem fallback.** A failed `git worktree remove` preserves the
   checkout. Cleanup never bypasses Git by deleting the directory directly.

6. **Repository-wide maintenance.** Preview clean, merged worktrees across
   legacy roots before applying cleanup:
   ```bash
   codetether worktree cleanup --base main \
     --root ../project-worktrees --root ../project.worktrees
   codetether worktree cleanup --base main \
     --root ../project-worktrees --root ../project.worktrees --apply
   ```
   Dirty, unmerged, locked, current, and primary worktrees are preserved.
   Local branches are also preserved so cleanup cannot erase committed work.

7. **Legacy managed cleanup.** Automatic Ralph/swarm cleanup removes a local
   branch only through Git's merged-branch check (`git branch -d`). Committed
   but unmerged branches remain available even after a clean checkout closes.

## Implementation

- `src/worktree/dirty_check.rs` — `is_worktree_dirty()` method
- `src/worktree/cleanup_remove.rs` — calls dirty check before `--force` removal
