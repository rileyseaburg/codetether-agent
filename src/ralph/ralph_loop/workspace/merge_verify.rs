//! Gate successful merges on story verification in the main workspace.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn verify_merged_story(
        &self,
        story: &UserStory,
        working_dir: &Path,
    ) -> bool {
        if let Err(error) = run_story_verification(working_dir, story).await {
            let error = format!("Verification steps failed after merge: {error}");
            warn!(target: "codetether_agent::ralph::ralph_loop::merge_verify", story_id = %story.id, error = %error);
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
