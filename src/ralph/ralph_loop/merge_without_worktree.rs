//! Verify and report a completed story that used the main working directory.
use super::*;
impl RalphLoop {
    pub(super) async fn complete_story_without_worktree(
        &mut self,
        story: &UserStory,
        working_dir: &Path,
    ) -> bool {
        // No worktree; verification still gates completion.
        if let Err(error) = run_story_verification(working_dir, story).await {
            let error = format!("Verification steps failed: {error}");
            warn!(story_id = %story.id, error = %error);
            self.record_story_result(story, false, Some(error));
            self.try_send_event(RalphEvent::StoryComplete {
                story_id: story.id.clone(),
                passed: false,
            });
            return false;
        }
        self.state.prd.mark_passed(&story.id);
        self.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: true,
        });

        // Record story result in store
        self.record_story_result(story, true, None);
        true
    }
}
