//! Git push and GitHub PR creation for TUI worktrees.
//!
//! After the agent finishes work in an isolated worktree,
//! this module stages, commits, pushes the branch, and
//! opens a pull request via the `gh` CLI.
//!
//! # Examples
//!
//! ```ignore
//! let url = push_and_create_pr(&worktree).await?;
//! ```

use crate::worktree::WorktreeInfo;

use super::pr_helpers::{create_github_pr, push_branch};

/// Push a worktree branch and open a GitHub pull request.
///
/// Stages any uncommitted changes, pushes to origin, then
/// invokes `gh pr create`.  Returns the new PR URL on
/// success.
///
/// # Examples
///
/// ```ignore
/// let url = push_and_create_pr(&worktree).await?;
/// ```
pub(super) async fn push_and_create_pr(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
) -> anyhow::Result<String> {
    stage_and_commit(wt).await;
    push_branch(wt).await?;
    create_github_pr(wt, base_branch).await
}

/// Stage and commit any uncommitted changes.
async fn stage_and_commit(wt: &WorktreeInfo) {
    let diff_check = tokio::process::Command::new("git")
        .args(["diff", "--quiet", "HEAD"])
        .current_dir(&wt.path)
        .status()
        .await;
    if diff_check.map(|s| !s.success()).unwrap_or(true) {
        let _ = tokio::process::Command::new("git")
            .args(["add", "-A"])
            .current_dir(&wt.path)
            .output()
            .await;
        let _ = tokio::process::Command::new("git")
            .args([
                "commit",
                "-m",
                &format!("codetether: TUI agent work ({})", wt.name),
            ])
            .current_dir(&wt.path)
            .output()
            .await;
    }
}
