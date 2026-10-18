//! Post-prompt worktree publication, local merge fallback and cleanup.
//! [handle_worktree_result] applies the user's PR intent only after success,
//! then cleans up the isolated worktree. [WorktreeState] carries its ownership.
mod merge;
mod result;
use codetether_worktree::{WorktreeInfo, WorktreeManager};
pub use result::handle_worktree_result;
/// Owned worktree manager, checkout and original target branch.
/// # Examples
/// ```no_run
/// # fn state(manager: codetether_worktree::WorktreeManager, checkout: codetether_worktree::WorktreeInfo) {
/// let state: codetether_github_pr::worktree::WorktreeState = (manager, checkout, None);
/// assert!(state.2.is_none());
/// # }
/// ```
pub type WorktreeState = (WorktreeManager, WorktreeInfo, Option<String>);
