//! Publication of sub-agent reasoning to the agent bus.

use super::state::State;

pub(super) fn publish<H: crate::Host>(state: &State<H>, thinking: &str) {
    codetether_swarm_bus::agent::publish_thinking(
        state.inner.bus.as_ref(),
        &state.inner.subtask_id,
        state.inner.conversation.steps,
        thinking,
    );
}
