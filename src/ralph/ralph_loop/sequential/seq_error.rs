//! Sequential Ralph error handling.
use super::*;
impl RalphLoop {
    pub(in crate::ralph::ralph_loop) fn record_sequential_error(
        &mut self,
        story: &UserStory,
        e: &anyhow::Error,
    ) {
        warn!(target: "codetether_agent::ralph::ralph_loop::seq_error", "LLM call failed: {}", e);
        let entry =
            codetether_ralph_core::progress::failed_entry(story, self.state.current_iteration, e);
        self.state.progress_log.push(entry);

        self.try_send_event(RalphEvent::StoryError {
            story_id: story.id.clone(),
            error: format!("{}", e),
        });
    }
}
