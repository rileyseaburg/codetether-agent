//! Push a worktree branch with its existing Git invocation.
use codetether_worktree::WorktreeInfo;

/// Push the worktree branch to origin.
///
/// Runs `git push -u origin <branch>` inside the worktree
/// directory and returns an error if the push fails.
///
/// # Arguments
/// * `wt` — wt input.
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
/// let _result = codetether_github_pr::worktree::helpers::push_branch(wt).await;
/// # }
/// ```
pub async fn push_branch(wt: &WorktreeInfo) -> anyhow::Result<()> {
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
