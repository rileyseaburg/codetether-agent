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

## Spawned child agents

`spawn_agent` and durable `agent action=spawn` allocate a fresh managed Git
worktree before the child's first turn. Writable ephemeral agents use the same
allocator; read-only ephemeral jobs retain a read-only tool registry and may
share the parent's checkout. Non-Git directories and ambiguous primary-checkout
layouts fail closed for writable children; there is no shared-write fallback.

This does not relax independent delegation policy. Mux-managed sessions can
still reject the legacy `agent action=spawn` route with
`MUX_AGENT_DELEGATION_FORBIDDEN` before allocation; that is not a lease conflict.

- Each child branches from its **immediate parent's committed HEAD**, resolved
  before allocation. A child of a child inherits that parent's commit, not the
  primary checkout's HEAD. Staged, unstaged, and untracked parent edits are not
  copied. Commit prerequisites before delegating tasks that depend on them.
- Checkouts live under the primary checkout's `.codetether-worktrees/`, with
  unique `child-<uuid>` names and `codetether/child-<uuid>` branches. The parent's
  relative working directory is preserved and must exist in the commit.
- The child session persists its own directory with `workspace_pinned = true`.
  Closing and resuming refreshes model/access settings without moving the child
  back into the parent's directory. Legacy unpinned sessions retain their
  existing workspace-refresh behavior; this does not migrate running children.
- Results include `isolation.checkout`: `workspace`, `worktree`, `branch`,
  `parent_workspace`, and `base_commit`. Successful creation does not mean the
  child's task succeeded; the original result's success flag is retained.
- There is no automatic merge or cleanup on completion. Failure after allocation
  retains the worktree for recovery; setup errors identify retained storage.

### Review and integrate explicitly

The child should commit only its task files and report the commit IDs plus
verification evidence. The parent reviews those commits, then deliberately
cherry-picks the selected task commits in order. For example, substitute the
returned checkout/branch/base and the child's reported commit ID:

```bash
git -C "$child_checkout" status --short
git -C "$parent_checkout" log --oneline "$base_commit..$child_branch"
git -C "$parent_checkout" diff "$base_commit...$child_branch"
git -C "$parent_checkout" show "$child_commit"
# After review, with a suitable parent working tree and normal write authority:
git -C "$parent_checkout" cherry-pick "$child_commit"
```

Lease enforcement remains enabled. Leases are keyed by canonical checkout
paths: a renewing parent-wide lease does not reserve separate child checkouts,
but writes targeting the parent's checkout still require the parent's lease.
Do not release or bypass a parent's lease merely to run isolated child work.

## Implementation

- `src/worktree/dirty_check.rs` — `is_worktree_dirty()` method
- `src/worktree/cleanup_remove.rs` — calls dirty check before `--force` removal