//! Report non-conflict merge failures and retain their cleanup policy.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn report_nonconflict_merge(
        &self,
        story: &UserStory,
        wt: &crate::worktree::WorktreeInfo,
        mgr: &Arc<WorktreeManager>,
        merge_result: &crate::worktree::MergeResult,
    ) {
        if merge_result.aborted {
            // Non-conflict failure that was already aborted
            warn!(target: "codetether_agent::ralph::ralph_loop::merge_nonconflict",
                story_id = %story.id,
                summary = %merge_result.summary,
                "Merge was aborted due to non-conflict failure"
            );
            // Cleanup worktree
            let _ = mgr.cleanup(&wt.name).await;
        } else {
            // Merge in progress state (should not reach here)
            warn!(target: "codetether_agent::ralph::ralph_loop::merge_nonconflict",
                story_id = %story.id,
                summary = %merge_result.summary,
                "Merge failed but not aborted - manual intervention may be needed"
            );
            // Don't cleanup - leave for debugging
        }
    }
}
