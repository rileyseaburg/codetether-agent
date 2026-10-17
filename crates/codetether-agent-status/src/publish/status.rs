//! Working and terminal task-state publication.
use crate::PublishHost;
use codetether_a2a_core::TaskState;
use codetether_bus_core::global::global;

/// Announce that a sub-agent was spawned (or is being messaged) and is now
/// working. The task id is the child session UUID, preventing cross-session collation.
/// Publish the existing working-state update when the bus and child exist.
/// # Arguments
/// * `agent_id` — Child live-registry key and sender ID.
/// * `summary` — Working-state summary.
/// # Returns
/// Unit; absent bus or child produces no publication.
/// # Examples
/// Use an application [PublishHost] implementation.
pub fn announce_working<H: PublishHost>(agent_id: &str, summary: impl Into<String>) {
    if let (Some(bus), Some(entry)) = (global(), H::get(agent_id)) {
        let task_id = crate::task_id(&entry.id);
        bus.handle(agent_id)
            .send_task_update(&task_id, TaskState::Working, Some(summary.into()));
    }
}

/// Announce that a sub-agent finished a message turn, with success/failure.
/// Publish the existing completed/failed update for a live child.
/// # Arguments
/// * `agent_id` — Child live-registry key and sender ID.
/// * `success` — Whether the child completed successfully.
/// * `summary` — Terminal activity summary.
/// # Returns
/// Unit; absent bus or child produces no publication.
/// # Examples
/// Use an application [PublishHost] implementation.
pub fn announce_done<H: PublishHost>(agent_id: &str, success: bool, summary: impl Into<String>) {
    if let (Some(bus), Some(entry)) = (global(), H::get(agent_id)) {
        let task_id = crate::task_id(&entry.id);
        let state = terminal_state(success);
        bus.handle(agent_id)
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
#[cfg(test)]
#[path = "../bus_publish_tests.rs"]
mod tests;
