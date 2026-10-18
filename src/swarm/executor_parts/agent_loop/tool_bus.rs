//! Publication of full tool output to the agent bus.

use super::state::State;

pub(super) fn publish(state: &State, name: &str, output: &str, success: bool) {
    codetether_swarm_bus::agent::publish_tool_output(
        state.bus.as_ref(),
        &state.subtask_id,
        state.conversation.steps,
        name,
        output,
        success,
    );
}
