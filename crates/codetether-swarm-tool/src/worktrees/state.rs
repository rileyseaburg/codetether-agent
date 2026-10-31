//! Ordered worktree allocation state and task expectation lookups.
use codetether_worktree::{WorktreeInfo, WorktreeManager};
use std::sync::Arc;

/// Worktree allocation state shared by swarm task runners.
///
/// `SwarmWorktrees` owns the worktree manager used for cleanup/lifecycle
/// operations and keeps the per-task worktree metadata in positional order.
/// Each slot in `infos` corresponds to a swarm participant index; `None` means
/// that participant did not need an isolated worktree. An error preserves failed allocation.
///
/// [`Self::create`] allocates isolated worktrees; finishing integrates successful
/// tasks while retaining failed worktrees for inspection. [`Self::shared`] reuses
/// the parent workspace for every worker; finishing leaves those edits in place
/// without committing, merging, or cleaning the shared checkout.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let worktrees = codetether_swarm_tool::worktrees::SwarmWorktrees::create(std::path::Path::new("."), &[]).await;
/// assert!(worktrees.expects_changes(0));
/// # }
/// ```
pub struct SwarmWorktrees {
    /// Parent workspace used directly by every worker in shared mode.
    pub(super) shared_root: Option<std::path::PathBuf>,
    /// Manager responsible for creating and cleaning up swarm worktrees.
    pub(super) mgr: Arc<WorktreeManager>,
    /// Per-participant worktree metadata, indexed by swarm task position.
    pub(super) infos: Vec<anyhow::Result<Option<WorktreeInfo>>>,
    pub(super) expects_changes: Vec<bool>,
    pub(super) verification: Vec<bool>,
}

impl SwarmWorktrees {
    /// Whether all participants can edit the same parent workspace.
    pub fn is_shared(&self) -> bool {
        self.shared_root.is_some()
    }
    /// Return metadata for a participant worktree.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based swarm participant index.
    ///
    /// # Returns
    ///
    /// Returns a borrowed [`WorktreeInfo`] when `index` is in range and that
    /// participant has an allocated worktree. Returns `None` when the index is
    /// out of range or the participant has no associated worktree.
    ///
    /// This method performs no filesystem access and has no side effects.
    pub(super) fn info(&self, index: usize) -> Option<&WorktreeInfo> {
        match self.infos.get(index) {
            Some(Ok(Some(info))) => Some(info),
            Some(Ok(None) | Err(_)) | None => None,
        }
    }

    /// Return the task change expectation, defaulting to true for missing indices.
    pub fn expects_changes(&self, index: usize) -> bool {
        self.expects_changes.get(index).copied().unwrap_or(true)
    }

    /// Return whether the task is verification, defaulting to false for missing indices.
    pub fn is_verification(&self, index: usize) -> bool {
        self.verification.get(index).copied().unwrap_or(false)
    }
}
