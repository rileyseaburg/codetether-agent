use super::super::request::wants_pr;
use super::{
    WorktreeState,
    merge::{merge_locally, push_or_merge},
};

/// Handle worktree publishing or local merge after a prompt.
///
/// Creates a PR only when the prompt explicitly asks for one;
/// otherwise merges the successful worktree locally and cleans up.
/// # Arguments
/// `success` controls integration; `worktree` owns the optional checkout;
/// `prompt` supplies the user's PR intent and metadata.
/// # Returns
/// Completes after best-effort publication/merge and cleanup, logging failures.
/// # Examples
/// ```no_run
/// # async fn finish(worktree: Option<codetether_github_pr::worktree::WorktreeState>) {
/// codetether_github_pr::worktree::handle_worktree_result(true, worktree, Some("Create a PR")).await;
/// # }
/// ```
pub async fn handle_worktree_result(
    success: bool,
    worktree: Option<WorktreeState>,
    prompt: Option<&str>,
) {
    let Some((mgr, wt, base_branch)) = worktree else {
        return;
    };
    if success {
        match prompt.filter(|p| wants_pr(p)) {
            Some(p) => push_or_merge(&mgr, &wt, base_branch.as_deref(), Some(p)).await,
            None => merge_locally(&mgr, &wt).await,
        }
    }
    if let Err(e) = mgr.cleanup(&wt.name).await {
        tracing::warn!(error = %e, "Failed to cleanup TUI worktree");
    }
}
