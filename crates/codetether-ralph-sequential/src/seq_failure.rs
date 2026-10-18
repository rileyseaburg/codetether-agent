//! Sequential Ralph failure handling.
use crate::*;
use codetether_ralph_core::events::RalphEvent;

pub(crate) fn record_quality_failure(
    host: &impl Host,
    story: &UserStory,
    entry_learnings: &[String],
) {
    warn!(target: "codetether_agent::ralph::ralph_loop::seq_failure", "Story {} failed quality checks", story.id);

    // Record failed story result in store
    host.record_story_result(story, false, Some("Quality checks failed".to_string()));

    // Publish failure learnings to bus
    let next = host.state().prd.next_story().map(|s| s.id.clone());
    host.bus_publish_story_result(
        story,
        host.state().current_iteration,
        entry_learnings,
        next.as_deref(),
    );

    host.try_send_event(RalphEvent::StoryComplete {
        story_id: story.id.clone(),
        passed: false,
    });
}
