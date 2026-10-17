//! Sequential Ralph unchecked handling.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) async fn finish_unchecked_story(
        &mut self,
        story: &UserStory,
    ) -> anyhow::Result<()> {
        // No quality checks; verification still gates completion.
        if let Err(error) = run_story_verification(&self.state.working_dir, story).await {
            let error = format!("Verification steps failed: {error}");
            warn!(target: "codetether_agent::ralph::ralph_loop::seq_unchecked", story_id = %story.id, error = %error);
            self.record_story_result(story, false, Some(error.clone()));
            self.try_send_event(RalphEvent::StoryComplete {
                story_id: story.id.clone(),
                passed: false,
            });
            return Ok(());
        }
        self.state.prd.mark_passed(&story.id);
        self.state.prd.save(&self.state.prd_path).await?;

        // Record story result in store
        self.record_story_result(story, true, None);

        self.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: true,
        });
        Ok(())
    }
}
