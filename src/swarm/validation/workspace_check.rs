use super::workspace_issues::{not_git_repo_issue, uncommitted_changes_issue, worktree_issue};
use super::workspace_status_build::workspace_status;
use super::{SwarmValidator, ValidationIssue, WorkspaceStatus};
use anyhow::Result;

impl SwarmValidator {
    pub(super) fn validate_workspace(
        &self,
        issues: &mut Vec<ValidationIssue>,
    ) -> Result<WorkspaceStatus> {
        let _current_dir = std::env::current_dir()?;
        let status = workspace_status(super::git_status::is_git_repo());
        push_workspace_issues(&status, issues);
        Ok(status)
    }
}

fn push_workspace_issues(status: &WorkspaceStatus, issues: &mut Vec<ValidationIssue>) {
    if !status.is_git_repo {
        issues.push(not_git_repo_issue());
    } else if status.uncommitted_changes > 0 {
        issues.push(uncommitted_changes_issue(status.uncommitted_changes));
    }
    if status.is_git_repo && !status.can_create_worktrees {
        issues.push(worktree_issue());
    }
}
