use super::WorktreeCleanupState;
use serde::Serialize;
use std::path::PathBuf;

/// One worktree and the safety decision made for it.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::worktree::maintenance::{WorktreeCleanupEntry, WorktreeCleanupState};
/// use std::path::PathBuf;
///
/// let entry = WorktreeCleanupEntry {
///     path: PathBuf::from("/repo-worktrees/done"),
///     branch: Some("feature/done".into()),
///     state: WorktreeCleanupState::Ready,
///     error: None,
/// };
/// assert_eq!(entry.state, WorktreeCleanupState::Ready);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct WorktreeCleanupEntry {
    /// Registered checkout path.
    pub path: PathBuf,
    /// Local branch, or `None` for a detached checkout.
    pub branch: Option<String>,
    /// Current safety decision or apply result.
    pub state: WorktreeCleanupState,
    /// Inspection or removal error when the operation could not complete.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}
