//! Gate successful merges on story verification in the main workspace.
use super::*;
impl RalphLoop {
    pub(super) async fn verify_merged_story(&self, story: &UserStory, working_dir: &Path) -> bool {
        if let Err(error) = run_story_verification(working_dir, story).await {
            let error = format!("Verification steps failed after merge: {error}");
            warn!(story_id = %story.id, error = %error);
            self.record_story_result(story, false, Some(error));
            self.try_send_event(RalphEvent::StoryComplete {
                story_id: story.id.clone(),
                passed: false,
            });
            return false;
        }
        true
    }
}
