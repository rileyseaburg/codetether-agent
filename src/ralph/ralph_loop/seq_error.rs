//! Sequential Ralph error handling.
use super::*;
impl RalphLoop {
    pub(super) fn record_sequential_error(&mut self, story: &UserStory, e: &anyhow::Error) {
        warn!("LLM call failed: {}", e);
        let entry =
            codetether_ralph_core::progress::failed_entry(story, self.state.current_iteration, e);
        self.state.progress_log.push(entry);

        self.try_send_event(RalphEvent::StoryError {
            story_id: story.id.clone(),
            error: format!("{}", e),
        });
    }
}
