//! Complete a resolved merge or abort it on completion failure.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn complete_resolved_merge(
        &mut self,
        working_dir: &Path,
        story: &UserStory,
        wt: &crate::worktree::WorktreeInfo,
        mgr: &Arc<WorktreeManager>,
    ) {
        // Complete the merge after resolution
        let commit_msg = format!("Merge: resolved conflicts for {}", story.id);
        match mgr.complete_merge(&wt.name, &commit_msg).await {
            Ok(final_result) => {
                if final_result.success {
                    self.verify_resolved_merge(working_dir, story).await;
                } else {
                    warn!(target: "codetether_agent::ralph::ralph_loop::conflict_complete",
                        story_id = %story.id,
                        "Merge failed even after resolution"
                    );
                    let _ = mgr.abort_merge(&wt.name).await;
                }
            }
            Err(e) => {
                warn!(target: "codetether_agent::ralph::ralph_loop::conflict_complete",
                    story_id = %story.id,
                    error = %e,
                    "Failed to complete merge after resolution"
                );
                let _ = mgr.abort_merge(&wt.name).await;
            }
        }
    }
}
