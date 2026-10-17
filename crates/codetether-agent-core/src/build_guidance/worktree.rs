//! Agent-facing guidance for managed worktree placement.

/// Guidance for placing managed worktrees under the selected workspace.
///
/// # Examples
/// ```
/// use codetether_agent_core::build_guidance::WORKTREE_GUIDANCE;
/// assert!(WORKTREE_GUIDANCE.contains(".codetether-worktrees"));
/// ```
pub const WORKTREE_GUIDANCE: &str = "

## Managed Worktree Location
- CodeTether-managed worktrees belong only under `<workspace-root>/.codetether-worktrees/`.
- Never create a worktree in `/tmp`, an OS temporary directory, a sibling directory, or another arbitrary location.
- Do not run `git worktree add` directly. Use CodeTether-managed worktree isolation so the runtime can enforce location, lifecycle, and cleanup rules.";
