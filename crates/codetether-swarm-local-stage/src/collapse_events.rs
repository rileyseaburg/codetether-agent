//! User-facing events for locally collapsed branches.

use super::state::State;
use codetether_swarm_core::SubTaskStatus;
use codetether_swarm_core::SwarmEvent;

pub(super) fn killed(state: &State<'_, impl crate::Host>, id: &str, reason: &str) {
    let Some(events) = &*state.executor.events() else {
        return;
    };
    let _ = events.try_send(SwarmEvent::SubTaskUpdate {
        id: id.into(),
        name: id.into(),
        status: SubTaskStatus::Cancelled,
        agent_name: Some(format!("agent-{id}")),
    });
    let _ = events.try_send(SwarmEvent::AgentError {
        subtask_id: id.into(),
        error: format!("Cancelled by collapse controller: {reason}"),
    });
}
