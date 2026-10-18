//! Collect commits and invoke GitHub pull-request creation.
use super::command::create_pr_args;
use super::description::{collect_commit_log, format_commit_bullets};
use codetether_worktree::WorktreeInfo;

/// Create a GitHub pull request via the `gh` CLI.
///
/// Collects the commit log from the worktree to build a
/// descriptive body.  Returns the URL of the new PR.
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
/// let _result = codetether_github_pr::worktree::helpers::create_github_pr(wt, None, None).await;
/// # }
/// ```
pub async fn create_github_pr(
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
) -> anyhow::Result<String> {
    let commits = collect_commit_log(&wt.path, base_branch).await;
    let bullets = format_commit_bullets(&commits);
    let args = create_pr_args(wt, base_branch, prompt, &bullets, &commits);
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
