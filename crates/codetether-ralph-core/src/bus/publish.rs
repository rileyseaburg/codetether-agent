//! Publish one story result as learning, optional handoff and progress messages.
use super::*;
/// Publish a completed attempt through the existing bus.
///
/// # Arguments
/// `state` supplies project progress; `story`, `iteration` and `learnings`
/// describe the attempt; `next_story_id` optionally requests a handoff.
/// # Returns
/// Publishes messages in learning, handoff, progress order.
/// # Examples
/// ```rust,no_run
/// fn example(bus: &std::sync::Arc<codetether_bus_core::AgentBus>, state: &codetether_ralph_core::RalphState, story: &codetether_ralph_core::UserStory) {
/// codetether_ralph_core::bus::publish(bus, state, story, 1, &[], None);
/// }
/// ```
pub fn publish(
    bus: &Arc<AgentBus>,
    state: &RalphState,
    story: &UserStory,
    iteration: usize,
    learnings: &[String],
    next_story_id: Option<&str>,
) {
    let prd_id = &state.prd.project;
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
        state.prd.passed_count(),
        state.prd.user_stories.len(),
        iteration,
        &format!("{:?}", state.status),
    );
}
