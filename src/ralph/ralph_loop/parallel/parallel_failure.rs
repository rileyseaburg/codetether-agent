//! Report a failed parallel task and retain its worktree for debugging.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn report_failed_parallel_story(
        &self,
        story: &UserStory,
        success: bool,
        failure_reason: Option<String>,
        worktree_info: Option<&crate::worktree::WorktreeInfo>,
    ) {
        // Failed - cleanup worktree without merging (keep for debugging)
        let error = codetether_ralph_core::outcome::story_failure(success, failure_reason);
        self.try_send_event(RalphEvent::StoryError {
            story_id: story.id.clone(),
            error: error.clone(),
        });
        self.record_story_result(story, false, Some(error));
        if let Some(wt) = worktree_info {
            info!(target: "codetether_agent::ralph::ralph_loop::parallel_failure",
                story_id = %story.id,
                worktree_path = %wt.path.display(),
                "Keeping worktree for debugging (story failed)"
            );
        }
    }
}
