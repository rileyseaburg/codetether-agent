//! Git push and GitHub PR creation helpers.
//!
//! Contains the push and PR creation steps used by
//! [`super::pr::push_and_create_pr`].

use super::pr_command::create_pr_args;
use super::pr_description::{collect_commit_log, format_commit_bullets};
use crate::worktree::WorktreeInfo;
#[path = "pr_process.rs"]
mod process;

/// Push the worktree branch to origin.
///
/// Runs `git push -u origin <branch>` inside the worktree
/// directory and returns an error if the push fails.
pub(super) async fn push_branch(wt: &WorktreeInfo, network_allowed: bool) -> anyhow::Result<()> {
    process::require_network(network_allowed)?;
    let args = ["push", "-u", "origin", &wt.branch].map(str::to_string);
    let output = crate::tool::git::process::output_networked(&wt.path, &args, &[], true)
        .await
        .map_err(|error| anyhow::anyhow!("Failed to run sandboxed git push: {error}"))?;
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
    network_allowed: bool,
) -> anyhow::Result<String> {
    let commits = collect_commit_log(&wt.path, base_branch).await;
    let bullets = format_commit_bullets(&commits);
    let args = create_pr_args(wt, base_branch, prompt, &bullets, &commits);
    process::require_network(network_allowed)?;
    let output = process::gh(&wt.path, &args).await?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow::anyhow!("gh pr create failed: {stderr}"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}
