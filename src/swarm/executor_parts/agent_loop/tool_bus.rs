//! Publication of full tool output to the agent bus.

use super::state::State;

pub(super) fn publish(state: &State, name: &str, output: &str, success: bool) {
    codetether_swarm_bus::agent::publish_tool_output(
        state.inner.bus.as_ref(),
        &state.inner.subtask_id,
        state.inner.conversation.steps,
        name,
        output,
        success,
    );
}
