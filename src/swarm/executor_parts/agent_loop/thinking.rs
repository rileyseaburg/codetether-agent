//! Publication of sub-agent reasoning to the agent bus.

use super::state::State;

pub(super) fn publish(state: &State, thinking: &str) {
    codetether_swarm_bus::agent::publish_thinking(
        state.bus.as_ref(),
        &state.subtask_id,
        state.steps,
        thinking,
    );
}
