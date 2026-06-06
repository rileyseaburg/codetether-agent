use crate::worktree::{WorktreeInfo, WorktreeManager};
use std::{path::PathBuf, sync::Arc};

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
    pub(super) infos: Vec<Option<WorktreeInfo>>,
}

impl SwarmWorktrees {
    /// Return the filesystem directory for a participant worktree.
    ///
    /// # Arguments
    ///
    /// * `index` - Zero-based swarm participant index.
    ///
    /// # Returns
    ///
    /// Returns `Some(PathBuf)` containing the worktree path when `index` is in
    /// range and that participant has an allocated worktree. Returns `None` when
    /// the index is out of range or the participant has no worktree.
    ///
    /// The returned path is cloned from the stored [`WorktreeInfo`], so callers
    /// may keep or modify the `PathBuf` value without mutating this state.
    pub(in crate::tool::swarm_execute) fn dir(&self, index: usize) -> Option<PathBuf> {
        self.infos
            .get(index)
            .and_then(|info| info.as_ref())
            .map(|info| info.path.clone())
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
        self.infos.get(index).and_then(Option::as_ref)
    }
}
