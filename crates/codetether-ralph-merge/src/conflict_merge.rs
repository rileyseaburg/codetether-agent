//! Handle resolver outcomes and perform the original final worktree cleanup.
use super::*;
pub(crate) async fn resolve_parallel_merge<H: Host>(
    host: &mut H,
    provider: &Arc<dyn Provider>,
    model: &str,
    working_dir: &Path,
    story: &UserStory,
    wt: &WorktreeInfo,
    mgr: &Arc<WorktreeManager>,
    merge_result: &MergeResult,
) {
    match invoke_merge_resolver(host, provider, model, working_dir, story, merge_result).await {
        Ok(resolved) => {
            if resolved {
                complete_resolved_merge(host, working_dir, story, wt, mgr).await;
            } else {
                warn!(target: "codetether_agent::ralph::ralph_loop::conflict_merge",
                    story_id = %story.id,
                    "Conflict resolver could not resolve all conflicts"
                );
                let _ = mgr.abort_merge(&wt.name).await;
            }
        }
        Err(e) => {
            warn!(target: "codetether_agent::ralph::ralph_loop::conflict_merge",
                story_id = %story.id,
                error = %e,
                "Conflict resolver failed"
            );
            let _ = mgr.abort_merge(&wt.name).await;
        }
    }
    // Cleanup worktree
    let _ = mgr.cleanup(&wt.name).await;
}
