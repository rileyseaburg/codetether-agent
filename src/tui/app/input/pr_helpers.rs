//! Git push and GitHub PR creation helpers (continued).
//!
//! Contains the push and PR creation steps used by
//! [`super::pr::push_and_create_pr`].
//!
//! # Examples
//!
//! ```ignore
//! push_branch(&wt).await?;
//! let url = create_github_pr(&wt).await?;
//! ```

use super::pr_command::create_pr_args;
use crate::worktree::WorktreeInfo;

/// Push the worktree branch to origin.
///
/// Runs `git push -u origin <branch>` inside the worktree
/// directory and returns an error if the push fails.
///
/// # Examples
///
/// ```ignore
/// push_branch(&wt).await?;
/// ```
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
/// Returns the URL of the newly created PR.
///
/// # Examples
///
/// ```ignore
/// let url = create_github_pr(&wt).await?;
/// ```
pub(super) async fn create_github_pr(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
) -> anyhow::Result<String> {
    let args = create_pr_args(wt, base_branch);
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
