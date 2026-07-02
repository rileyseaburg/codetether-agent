# Worktree Lifecycle for Multi-Edit Agent Tasks

> Issue #297 Part B — worktree churn destroying in-flight edits.

## Problem

During multi-step agent tasks, per-turn worktree isolation could tear down
the working directory mid-task, silently discarding uncommitted edits.

## Lifecycle Contract

1. **One task = one worktree.** A single agent task operates against a stable
   worktree for its full duration. The worktree must not be torn down between
   turns or steps of the same task.

2. **Dirty-check guard.** Before `git worktree remove --force`, the
   `WorktreeManager` checks `git status --porcelain`. If uncommitted changes
   exist, removal is **refused** and an error is logged:
   ```
   Refusing to force-remove dirty worktree — commit or stash first.
   ```

3. **Cleanup responsibility.** The caller (CLI command, TUI, or agent loop)
   is responsible for committing or stashing before requesting worktree
   cleanup. The worktree manager will not silently destroy work.

4. **Manual override.** If a caller truly needs to discard a worktree
   regardless of dirty state (e.g., abandoned task), they should explicitly
   commit or `git stash` first, or use `cleanup_all()` after confirming no
   in-flight work exists.

## Implementation

- `src/worktree/dirty_check.rs` — `is_worktree_dirty()` method
- `src/worktree/cleanup_remove.rs` — calls dirty check before `--force` removal
