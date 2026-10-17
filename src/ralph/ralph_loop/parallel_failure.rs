//! Report a failed parallel task and retain its worktree for debugging.
use super::*;
impl RalphLoop {
    pub(super) fn report_failed_parallel_story(
        &self,
        story: &UserStory,
        success: bool,
        failure_reason: Option<String>,
        worktree_info: Option<&crate::worktree::WorktreeInfo>,
    ) {
        // Failed - cleanup worktree without merging (keep for debugging)
        let error = failure_reason.unwrap_or_else(|| {
            if success {
                "Quality or verification checks failed".to_string()
            } else {
                "LLM call failed".to_string()
            }
        });
        self.try_send_event(RalphEvent::StoryError {
            story_id: story.id.clone(),
            error: error.clone(),
        });
        self.record_story_result(story, false, Some(error));
        if let Some(wt) = worktree_info {
            info!(
                story_id = %story.id,
                worktree_path = %wt.path.display(),
                "Keeping worktree for debugging (story failed)"
            );
        }
    }
}
