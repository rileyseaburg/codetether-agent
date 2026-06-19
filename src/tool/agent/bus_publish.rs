//! Publishes `agent` tool activity to the shared [`AgentBus`] so sub-agents
//! spawned or messaged by a parent agent are visible in the TUI Bus view and
//! to peer agents — not just buried in the tool result JSON.

use crate::a2a::types::TaskState;
use crate::bus::global;

/// Announce that a sub-agent was spawned (or is being messaged) and is now
/// working. The task id is the agent name so updates collate per sub-agent.
pub(super) fn announce_working(agent: &str, summary: impl Into<String>) {
    if let Some(bus) = global() {
        bus.handle(agent)
            .send_task_update(agent, TaskState::Working, Some(summary.into()));
    }
}

/// Announce that a sub-agent finished a message turn, with success/failure.
pub(super) fn announce_done(agent: &str, success: bool, summary: impl Into<String>) {
    if let Some(bus) = global() {
        let state = if success {
            TaskState::Completed
        } else {
            TaskState::Failed
        };
        bus.handle(agent)
            .send_task_update(agent, state, Some(summary.into()));
    }
}
