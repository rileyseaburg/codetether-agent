//! Sequential Ralph failure handling.
use super::*;
impl RalphLoop {
    pub(super) fn record_quality_failure(&self, story: &UserStory, entry_learnings: &[String]) {
        warn!("Story {} failed quality checks", story.id);

        // Record failed story result in store
        self.record_story_result(story, false, Some("Quality checks failed".to_string()));

        // Publish failure learnings to bus
        let next = self.state.prd.next_story().map(|s| s.id.clone());
        self.bus_publish_story_result(
            story,
            self.state.current_iteration,
            entry_learnings,
            next.as_deref(),
        );

        self.try_send_event(RalphEvent::StoryComplete {
            story_id: story.id.clone(),
            passed: false,
        });
    }
}
