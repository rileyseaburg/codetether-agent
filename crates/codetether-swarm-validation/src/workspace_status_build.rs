use super::WorkspaceStatus;

pub(super) fn workspace_status(is_git_repo: bool) -> WorkspaceStatus {
    let status = is_git_repo.then(super::git_status::status_porcelain);
    WorkspaceStatus {
        is_git_repo,
        current_branch: is_git_repo
            .then(super::git_status::current_branch)
            .flatten(),
        uncommitted_changes: status.as_deref().map(count_changes).unwrap_or_default(),
        has_untracked_files: status.as_deref().map(has_untracked).unwrap_or_default(),
        can_create_worktrees: is_git_repo && super::git_status::can_create_worktrees(),
    }
}

fn count_changes(status: &str) -> usize {
    status.lines().filter(|line| !line.is_empty()).count()
}

fn has_untracked(status: &str) -> bool {
    status.lines().any(|line| line.starts_with("??"))
}
