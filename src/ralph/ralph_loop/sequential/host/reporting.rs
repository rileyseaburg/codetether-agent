//! Bind nonblocking events, store updates and progress-file delivery.
use super::*;
impl codetether_ralph_sequential::Reporting for RalphLoop {
    fn try_send_event(&self, event: RalphEvent) {
        self.try_send_event(event);
    }
    fn record_story_result(&self, story: &UserStory, passed: bool, error: Option<String>) {
        self.record_story_result(story, passed, error);
    }
    fn bus_publish_story_result(
        &self,
        story: &UserStory,
        iteration: usize,
        learnings: &[String],
        next: Option<&str>,
    ) {
        self.bus_publish_story_result(story, iteration, learnings, next);
    }
    fn append_progress(&self, entry: &ProgressEntry, response: &str) -> anyhow::Result<()> {
        self.append_progress(entry, response)
    }
}
