//! Agent-facing guidance for managed worktree placement.

pub(crate) const WORKTREE_GUIDANCE: &str = "

## Managed Worktree Location
- CodeTether-managed worktrees belong only under `<workspace-root>/.codetether-worktrees/`.
- Never create a worktree in `/tmp`, an OS temporary directory, a sibling directory, or another arbitrary location.
- Do not run `git worktree add` directly. Use CodeTether-managed worktree isolation so the runtime can enforce location, lifecycle, and cleanup rules.";
