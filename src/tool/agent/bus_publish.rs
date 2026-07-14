//! Publishes `agent` tool activity to the shared [`AgentBus`] so sub-agents
//! spawned or messaged by a parent agent are visible in the TUI Bus view and
//! to peer agents — not just buried in the tool result JSON.

use crate::a2a::types::{Part, TaskState};
use crate::bus::global;

use super::store;

#[path = "lifecycle.rs"]
pub(super) mod lifecycle;
#[cfg(test)]
#[path = "bus_publish_tests.rs"]
mod tests;

/// Announce that a sub-agent was spawned (or is being messaged) and is now
/// working. The task id is the child session UUID, preventing cross-session collation.
pub(super) fn announce_working(agent: &str, summary: impl Into<String>) {
    if let (Some(bus), Some(entry)) = (global(), store::get(agent)) {
        let task_id = lifecycle::task_id(&entry.session);
        bus.handle(agent)
            .send_task_update(&task_id, TaskState::Working, Some(summary.into()));
    }
}

/// Announce that a sub-agent finished a message turn, with success/failure.
pub(super) fn announce_done(agent: &str, success: bool, summary: impl Into<String>) {
    if let (Some(bus), Some(entry)) = (global(), store::get(agent)) {
        let task_id = lifecycle::task_id(&entry.session);
        let state = terminal_state(success);
        bus.handle(agent)
            .send_task_update(&task_id, state, Some(summary.into()));
    }
}

fn terminal_state(success: bool) -> TaskState {
    if success {
        TaskState::Completed
    } else {
        TaskState::Failed
    }
}

/// Deliver the completed child response directly to its owning parent session.
pub(super) fn announce_result(agent: &str, response: &str, error: Option<&str>) {
    let (Some(bus), Some(entry)) = (global(), store::get(agent)) else {
        return;
    };
    let Some(parent) = entry.owner_session_id.as_deref() else {
        return;
    };
    let text = lifecycle::result_message(agent, response, error);
    bus.handle(agent)
        .send_to_agent(parent, vec![Part::Text { text }]);
}
