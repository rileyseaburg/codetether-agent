/// Workspace and git state summary.
#[derive(Debug, Clone)]
///
/// # Examples
/// ```
/// let status = codetether_swarm_core::validation::WorkspaceStatus { is_git_repo: true, current_branch: Some("main".into()), uncommitted_changes: 0, has_untracked_files: false, can_create_worktrees: true };
/// assert!(status.can_create_worktrees);
/// ```
pub struct WorkspaceStatus {
    /// Whether the current directory is inside a git repository.
    pub is_git_repo: bool,
    /// Current branch name when available.
    pub current_branch: Option<String>,
    /// Number of uncommitted changes.
    pub uncommitted_changes: usize,
    /// Whether untracked files are present.
    pub has_untracked_files: bool,
    /// Whether git worktrees appear usable.
    pub can_create_worktrees: bool,
}
