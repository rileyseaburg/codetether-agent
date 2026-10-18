//! Isolated prompt worktrees independent of frontend session ownership.
//! [create_worktree] creates a checkout and updates the host's effective
//! directory through a callback. [WorktreeState] retains cleanup ownership.
mod create;
mod frontend;
pub use create::create_worktree;
/// Owned worktree manager, checkout and original target branch.
/// # Examples
/// ```no_run
/// # fn state(manager: codetether_worktree::WorktreeManager, checkout: codetether_worktree::WorktreeInfo) {
/// let state: codetether_worktree::prompt::WorktreeState = (manager, checkout, None);
/// assert!(state.2.is_none());
/// # }
/// ```
pub type WorktreeState = (crate::WorktreeManager, crate::WorktreeInfo, Option<String>);
