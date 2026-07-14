//! Publication of full tool output to the agent bus.

use super::state::State;
use crate::bus::BusMessage;

pub(super) fn publish(state: &State, name: &str, output: &str, success: bool) {
    let Some(bus) = &state.bus else { return };
    bus.handle(&state.subtask_id).send(
        format!("tools.{name}"),
        BusMessage::ToolOutputFull {
            agent_id: state.subtask_id.clone(),
            tool_name: name.into(),
            output: crate::swarm::live_bus::tool_output(output),
            success,
            step: state.steps,
        },
    );
}
