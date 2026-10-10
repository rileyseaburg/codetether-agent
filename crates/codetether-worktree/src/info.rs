use std::path::PathBuf;

/// Information about one isolated Git worktree.
#[derive(Debug, Clone)]
pub struct WorktreeInfo {
    /// Worktree name or identifier.
    pub name: String,
    /// Filesystem path to the worktree checkout.
    pub path: PathBuf,
    /// Branch checked out by the worktree.
    pub branch: String,
    /// Whether the manager considers this worktree active.
    #[allow(dead_code)]
    pub active: bool,
}

/// Result from merging a worktree branch back into the repository.
#[derive(Debug, Clone)]
pub struct MergeResult {
    /// Whether the merge completed successfully.
    pub success: bool,
    /// Whether the merge was aborted before completion.
    pub aborted: bool,
    /// Files reported as conflicted by Git.
    pub conflicts: Vec<String>,
    /// Diffs for conflicted files.
    pub conflict_diffs: Vec<(String, String)>,
    /// Number of files changed by the final merge commit.
    pub files_changed: usize,
    /// Human-readable merge summary.
    pub summary: String,
}
