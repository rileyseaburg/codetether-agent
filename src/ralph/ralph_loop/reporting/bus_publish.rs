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
        let prd_id = &self.state.prd.project;
        let handle = bus.handle(format!("ralph.{}", story.id));

        // Publish learnings
        handle.publish_ralph_learning(
            prd_id,
            &story.id,
            iteration,
            learnings.to_vec(),
            serde_json::json!({
                "story_title": story.title,
                "passed": story.passes,
            }),
        );

        // Publish handoff to next story if applicable
        if let Some(next_id) = next_story_id {
            handle.publish_ralph_handoff(
                prd_id,
                &story.id,
                next_id,
                serde_json::json!({ "learnings": learnings }),
                &format!(
                    "{} {} (iteration {})",
                    story.id,
                    if story.passes { "passed" } else { "failed" },
                    iteration
                ),
            );
        }

        // Publish progress
        handle.publish_ralph_progress(
            prd_id,
            self.state.prd.passed_count(),
            self.state.prd.user_stories.len(),
            iteration,
            &format!("{:?}", self.state.status),
        );
    }
}
