//! Worktree merge fallback helpers.
//!
//! Attempts to push a PR, and falls back to a local merge
//! when the push fails.
//!
//! # Examples
//!
//! ```ignore
//! push_or_merge(&mgr, &wt).await;
//! ```

use crate::worktree::{WorktreeInfo, WorktreeManager};

use super::pr::push_and_create_pr;

/// Try pushing a PR; fall back to local merge on failure.
///
/// # Examples
///
/// ```ignore
/// push_or_merge(&mgr, &wt).await;
/// ```
pub(super) async fn push_or_merge(
    mgr: &WorktreeManager,
    wt: &WorktreeInfo,
    base_branch: Option<&str>,
) {
    match push_and_create_pr(wt, base_branch).await {
        Ok(pr_url) => {
            tracing::info!(
                worktree = %wt.name, branch = %wt.branch,
                pr_url = %pr_url, "Pushed branch and created GitHub PR"
            );
        }
        Err(e) => {
            tracing::warn!(error = %e, "Failed to push/create PR, falling back to local merge");
            fallback_merge(mgr, wt).await;
        }
    }
}

/// Attempt a local git merge as a fallback.
async fn fallback_merge(mgr: &WorktreeManager, wt: &WorktreeInfo) {
    match mgr.merge(&wt.name).await {
        Ok(mr) if mr.success => {
            tracing::info!(
                worktree = %wt.name,
                files_changed = mr.files_changed,
                "Fallback: auto-merged TUI worktree locally"
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
