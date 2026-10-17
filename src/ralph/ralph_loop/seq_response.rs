//! Sequential Ralph response handling.
use super::*;
impl RalphLoop {
    pub(super) async fn handle_sequential_response(
        &mut self,
        story: &UserStory,
        response: &str,
    ) -> anyhow::Result<()> {
        // Log progress
        let entry = ProgressEntry {
            story_id: story.id.clone(),
            iteration: self.state.current_iteration,
            status: "completed".to_string(),
            learnings: self.extract_learnings(response),
            files_changed: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.append_progress(&entry, response)?;
        let entry_learnings = entry.learnings.clone();
        self.state.progress_log.push(entry);

        if self.config.quality_checks_enabled {
            if self.run_quality_gates_with_events(&story.id).await? {
                self.finish_verified_story(story, &entry_learnings).await?;
            } else {
                self.record_quality_failure(story, &entry_learnings);
            }
        } else {
            self.finish_unchecked_story(story).await?;
        }
        Ok(())
    }
}
