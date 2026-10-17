//! Sequential Ralph error handling.
use super::*;
impl RalphLoop {
    pub(super) fn record_sequential_error(&mut self, story: &UserStory, e: &anyhow::Error) {
        warn!("LLM call failed: {}", e);
        let entry = ProgressEntry {
            story_id: story.id.clone(),
            iteration: self.state.current_iteration,
            status: format!("failed: {}", e),
            learnings: Vec::new(),
            files_changed: Vec::new(),
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.state.progress_log.push(entry);

        self.try_send_event(RalphEvent::StoryError {
            story_id: story.id.clone(),
            error: format!("{}", e),
        });
    }
}
