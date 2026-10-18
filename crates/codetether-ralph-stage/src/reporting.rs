//! Nonblocking lifecycle, persistence and bus delivery for a stage.
use crate::*;
pub(crate) fn try_send_event<H: Host>(host: &H, event: RalphEvent) {
    if let Some(tx) = host.inputs().event_tx {
        let _ = tx.try_send(event);
    }
}
pub(crate) fn bus_publish_story_result<H: Host>(
    host: &H,
    story: &UserStory,
    iteration: usize,
    learnings: &[String],
    next_story_id: Option<&str>,
) {
    let Some(bus) = host.inputs().bus else { return };
    codetether_ralph_core::bus::publish(
        bus,
        host.inputs().state,
        story,
        iteration,
        learnings,
        next_story_id,
    );
}
pub(crate) fn bus_collect_learnings<H: Host>(host: &H) -> Vec<String> {
    let Some(bus) = host.inputs().bus else {
        return Vec::new();
    };
    codetether_ralph_core::bus::collect(bus, &host.inputs().state.prd.project)
}
pub(crate) fn record_story_result<H: Host>(
    host: &H,
    story: &UserStory,
    passed: bool,
    error: Option<String>,
) {
    codetether_ralph_core::state_store::updates::story(
        host.inputs().store,
        host.inputs().run_id,
        host.inputs().state,
        story,
        passed,
        error,
    );
}
