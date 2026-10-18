//! Commit a successful task and merge its isolated workspace if present.
use super::*;
/// Commit and integrate one verified story, then apply its completion outcome.
/// # Arguments
/// Receives the native host, selected model/provider, main workspace, story and
/// optional isolated worktree and its manager.
/// # Returns
/// Returns after the selected merge, verification and cleanup path finishes.
/// Failures are reported through the host and existing tracing targets.
/// # Examples
/// ```rust,no_run
/// # async fn example<H: codetether_ralph_merge::Host>(host: &mut H, provider: &std::sync::Arc<dyn codetether_provider_core::Provider>, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_merge::merge_parallel_story(host, provider, "model", std::path::Path::new("."), story, None, None).await;
/// # }
/// ```
pub async fn merge_parallel_story<H: Host>(
    host: &mut H,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story: &UserStory,
    worktree_info: Option<&WorktreeInfo>,
    worktree_mgr: Option<&Arc<WorktreeManager>>,
) {
    info!(target: "codetether_agent::ralph::ralph_loop::merge_story", "Story {} passed quality checks!", story.id);

    // Commit in worktree first
    if let Some(wt) = worktree_info {
        let _ = host.commit_in_dir(&wt.path, story);
    }

    // Merge worktree back to main
    if let (Some(wt), Some(mgr)) = (worktree_info, worktree_mgr) {
        let ctx = MergeContext {
            provider,
            model,
            working_dir,
            story,
            wt,
            mgr,
        };
        let result = mgr.merge(&wt.name).await;
        apply_worktree_merge(host, &ctx, result).await;
    } else {
        if !complete_story_without_worktree(host, story, working_dir).await {
            return;
        }
    }
}
