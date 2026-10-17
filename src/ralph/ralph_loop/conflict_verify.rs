//! Verify the main workspace after a resolved merge is completed.
use super::*;
impl RalphLoop {
    pub(super) async fn verify_resolved_merge(&mut self, working_dir: &Path, story: &UserStory) {
        info!(
            story_id = %story.id,
            "Merge completed after conflict resolution"
        );
        match run_story_verification(working_dir, story).await {
            Ok(()) => self.state.prd.mark_passed(&story.id),
            Err(e) => {
                let error = format!("Verification failed after conflict resolution: {e}");
                warn!(
                    story_id = %story.id,
                    error = %error
                );
                self.record_story_result(story, false, Some(error));
                self.try_send_event(RalphEvent::StoryComplete {
                    story_id: story.id.clone(),
                    passed: false,
                });
            }
        }
    }
}
