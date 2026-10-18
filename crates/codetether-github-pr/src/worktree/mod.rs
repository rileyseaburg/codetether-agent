//! Git push and GitHub PR creation for TUI worktrees.
//!
//! After the agent finishes work in an isolated worktree,
//! this module stages, commits, pushes the branch, and
//! opens a pull request via the `gh` CLI.

use codetether_worktree::WorktreeInfo;

use self::helpers::{create_github_pr, push_branch};

#[cfg(test)]
mod base_tests;
pub mod body;
pub mod command;
mod create;
pub mod description;
pub mod helpers;
mod push;
pub mod request;
pub mod title;

/// Push a worktree branch and open a GitHub pull request.
///
/// Stages any uncommitted changes, pushes to origin, then
/// invokes `gh pr create`.  Returns the new PR URL on
/// success.
///
/// # Arguments
/// * `wt` — wt input.
/// * `base_branch` — base branch input.
/// * `prompt` — prompt input.
///
/// # Returns
/// The operation result or its reported error.
///
/// # Errors
/// Returns process-launch errors or failed Git/GitHub command details.
///
/// # Examples
/// ```no_run
/// # async fn example(wt: &codetether_worktree::WorktreeInfo) {
/// let _result = codetether_github_pr::worktree::push_and_create_pr(wt, None, None).await;
/// # }
/// ```
pub async fn push_and_create_pr(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
) -> anyhow::Result<String> {
    stage_and_commit(wt).await;
    push_branch(wt).await?;
    create_github_pr(wt, base_branch, prompt).await
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
