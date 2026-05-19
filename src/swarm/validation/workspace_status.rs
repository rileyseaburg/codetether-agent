/// Workspace and git state summary.
#[derive(Debug, Clone)]
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
