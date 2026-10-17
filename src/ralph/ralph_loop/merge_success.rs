//! Mark and report a verified successful worktree merge, then clean up.
use super::*;
impl RalphLoop {
    pub(super) async fn complete_successful_merge(
        &mut self,
        story: &UserStory,
        working_dir: &Path,
        wt: &crate::worktree::WorktreeInfo,
        mgr: &Arc<WorktreeManager>,
        merge_result: &crate::worktree::MergeResult,
    ) -> bool {
        info!(
            story_id = %story.id,
            files_changed = merge_result.files_changed,
            "Merged story changes successfully"
        );
        if !self.verify_merged_story(story, working_dir).await {
            let _ = mgr.cleanup(&wt.name).await;
            return false;
        }
        self.state.prd.mark_passed(&story.id);
        self.try_send_event(RalphEvent::StoryMerge {
            story_id: story.id.clone(),
            success: true,
            summary: merge_result.summary.clone(),
        });
        self.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: true,
        });

        // Record story result in store
        self.record_story_result(story, true, None);

        // Cleanup worktree
        let _ = mgr.cleanup(&wt.name).await;
        true
    }
}
