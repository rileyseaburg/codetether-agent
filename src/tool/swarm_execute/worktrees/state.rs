use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::sync::Arc;

/// Worktree allocation state shared by swarm task runners.
///
/// `SwarmWorktrees` owns the worktree manager used for cleanup/lifecycle
/// operations and keeps the per-task worktree metadata in positional order.
/// Each slot in `infos` corresponds to a swarm participant index; `None` means
/// that participant did not receive an isolated worktree or allocation failed.
///
/// The struct does not create, delete, or mutate worktrees itself. It is a small
/// lookup container used by orchestration code after worktree setup has already
/// completed.
pub(in crate::tool::swarm_execute) struct SwarmWorktrees {
    /// Manager responsible for creating and cleaning up swarm worktrees.
    pub(super) mgr: Arc<WorktreeManager>,
    /// Per-participant worktree metadata, indexed by swarm task position.
    pub(super) infos: Vec<anyhow::Result<Option<WorktreeInfo>>>,
    pub(super) expects_changes: Vec<bool>,
    pub(super) verification: Vec<bool>,
}

impl SwarmWorktrees {
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

    pub(in crate::tool::swarm_execute) fn expects_changes(&self, index: usize) -> bool {
        self.expects_changes.get(index).copied().unwrap_or(true)
    }

    pub(in crate::tool::swarm_execute) fn is_verification(&self, index: usize) -> bool {
        self.verification.get(index).copied().unwrap_or(false)
    }
}
