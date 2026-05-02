use super::{IssueCategory, IssueSeverity, ValidationIssue};

pub(super) fn not_git_repo_issue() -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Warning,
        category: IssueCategory::Workspace,
        message: "Not in a git repository. Worktree isolation will be disabled.".into(),
        suggestion: Some(
            "Initialize a git repository with 'git init' or run from a git repo".into(),
        ),
    }
}

pub(super) fn uncommitted_changes_issue(count: usize) -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Warning,
        category: IssueCategory::Workspace,
        message: format!("Found {count} uncommitted change(s) in the working directory"),
        suggestion: Some("Consider committing or stashing changes before running swarm".into()),
    }
}

pub(super) fn worktree_issue() -> ValidationIssue {
    ValidationIssue {
        severity: IssueSeverity::Warning,
        category: IssueCategory::Workspace,
        message: "Unable to verify worktree creation capability".into(),
        suggestion: Some("Ensure git worktree is available and you have proper permissions".into()),
    }
}
