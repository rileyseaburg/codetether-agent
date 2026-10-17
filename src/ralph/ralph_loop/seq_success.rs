//! Sequential Ralph success handling.
use super::*;
impl RalphLoop {
    pub(super) async fn finish_verified_story(
        &mut self,
        story: &UserStory,
        entry_learnings: &[String],
    ) -> anyhow::Result<()> {
        info!("Story {} passed quality checks!", story.id);
        if let Err(error) = run_story_verification(&self.state.working_dir, story).await {
            let error = format!("Verification steps failed: {error}");
            warn!(story_id = %story.id, error = %error);
            self.record_story_result(story, false, Some(error.clone()));
            self.try_send_event(RalphEvent::StoryComplete {
                story_id: story.id.clone(),
                passed: false,
            });
            return Ok(());
        }
        self.state.prd.mark_passed(&story.id);

        // Record story result in store
        self.record_story_result(story, true, None);

        self.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: true,
        });

        // Publish learnings to bus
        let next = self.state.prd.next_story().map(|s| s.id.clone());
        self.bus_publish_story_result(
            story,
            self.state.current_iteration,
            entry_learnings,
            next.as_deref(),
        );

        // Commit changes
        if self.config.auto_commit {
            self.commit_story(story)?;
        }

        // Save updated PRD
        self.state.prd.save(&self.state.prd_path).await?;
        Ok(())
    }
}
