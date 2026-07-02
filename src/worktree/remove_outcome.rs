//! Outcome of a worktree removal attempt (issue #297 Part B review).
//!
//! Callers must not delete branches or untrack a worktree that was not
//! actually removed (e.g. refused because it was dirty), otherwise the
//! manager forgets a worktree that still exists on disk and in Git.

/// Result of [`super::WorktreeManager::remove_worktree`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RemoveOutcome {
    /// The worktree directory was removed successfully.
    Removed,
    /// Removal was refused because the worktree had uncommitted changes.
    RefusedDirty,
}

impl RemoveOutcome {
    /// Whether the worktree was actually removed (safe to untrack / delete branch).
    pub(crate) fn removed(self) -> bool {
        matches!(self, RemoveOutcome::Removed)
    }
}
