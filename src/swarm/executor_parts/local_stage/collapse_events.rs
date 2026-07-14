//! User-facing events for locally collapsed branches.

use super::state::State;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn killed(state: &State<'_>, id: &str, reason: &str) {
    let Some(events) = &state.executor.event_tx else {
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
