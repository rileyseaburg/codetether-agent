//! Publish story learnings, handoffs and progress to the agent bus.
use super::*;
impl RalphLoop {
    /// Publish story learnings, handoff context, and progress to the bus.
    pub(in crate::ralph::ralph_loop) fn bus_publish_story_result(
        &self,
        story: &UserStory,
        iteration: usize,
        learnings: &[String],
        next_story_id: Option<&str>,
    ) {
        let Some(ref bus) = self.bus else { return };
        codetether_ralph_core::bus::publish(
            bus,
            &self.state,
            story,
            iteration,
            learnings,
            next_story_id,
        );
    }
}
