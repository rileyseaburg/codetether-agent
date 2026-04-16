//! Worktree merge recovery helpers.
//!
//! Attempts to push a PR, and recovers with a local merge
//! when the push fails.

use crate::worktree::{WorktreeInfo, WorktreeManager};

use super::pr::push_and_create_pr;

/// Try pushing a PR; recover with local merge on failure.
///
/// Passes the original `prompt` through so the PR title
/// and body reflect the user's intent.
pub(super) async fn push_or_merge(
    mgr: &WorktreeManager,
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
    prompt: Option<&str>,
) {
    match push_and_create_pr(wt, base_branch, prompt).await {
        Ok(pr_url) => {
            tracing::info!(
                worktree = %wt.name, branch = %wt.branch,
                pr_url = %pr_url, "Pushed branch and created GitHub PR"
            );
        }
        Err(e) => {
            tracing::warn!(error = %e, "PR creation failed, attempting local merge");
            recover_with_local_merge(mgr, wt).await;
        }
    }
}

/// Attempt a local git merge as a recovery path.
async fn recover_with_local_merge(mgr: &WorktreeManager, wt: &WorktreeInfo) {
    match mgr.merge(&wt.name).await {
        Ok(mr) if mr.success => {
            tracing::info!(
                worktree = %wt.name,
                files_changed = mr.files_changed,
                "Recovered: auto-merged TUI worktree locally"
            );
        }
        Ok(mr) => {
            tracing::warn!(
                worktree = %wt.name, conflicts = ?mr.conflicts,
                "TUI worktree merge had conflicts"
            );
        }
        Err(e) => tracing::warn!(error = %e, "Failed to merge TUI worktree"),
    }
}
