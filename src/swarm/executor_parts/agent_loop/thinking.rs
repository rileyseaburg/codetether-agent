//! Publication of sub-agent reasoning to the agent bus.

use super::state::State;
use crate::bus::BusMessage;

pub(super) fn publish(state: &State, thinking: &str) {
    if thinking.is_empty() {
        return;
    }
    let Some(bus) = &state.bus else { return };
    bus.handle(&state.subtask_id).send(
        format!("agent.{}.thinking", state.subtask_id),
        BusMessage::AgentThinking {
            agent_id: state.subtask_id.clone(),
            thinking: crate::swarm::live_bus::thinking(thinking),
            step: state.steps,
        },
    );
}
