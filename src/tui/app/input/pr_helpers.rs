//! Git push and GitHub PR creation helpers.
//!
//! Contains the push and PR creation steps used by
//! [`super::pr::push_and_create_pr`].

use super::pr_command::create_pr_args;
use super::pr_description::{collect_commit_log, format_commit_bullets};
use crate::worktree::WorktreeInfo;

/// Push the worktree branch to origin.
///
/// Runs `git push -u origin <branch>` inside the worktree
/// directory and returns an error if the push fails.
pub(super) async fn push_branch(wt: &WorktreeInfo) -> anyhow::Result<()> {
    let output = tokio::process::Command::new("git")
        .args(["push", "-u", "origin", &wt.branch])
        .current_dir(&wt.path)
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run git push: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("git push failed: {stderr}"));
    }
    tracing::info!(branch = %wt.branch, "Pushed branch to origin");
    Ok(())
}

/// Create a GitHub pull request via the `gh` CLI.
///
/// Collects the commit log from the worktree to build a
/// descriptive body.  Returns the URL of the new PR.
pub(super) async fn create_github_pr(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
) -> anyhow::Result<String> {
    let commits = collect_commit_log(&wt.path, base_branch).await;
    let bullets = format_commit_bullets(&commits);
    let args = create_pr_args(wt, base_branch, prompt, &bullets);
    let output = tokio::process::Command::new("gh")
        .args(&args)
        .current_dir(&wt.path)
        .output()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run gh pr create: {e}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("gh pr create failed: {stderr}"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
