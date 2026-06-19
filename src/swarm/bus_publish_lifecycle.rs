//! Lifecycle (task-state) swarm events mapped to bus task updates.

use crate::a2a::types::TaskState;
use crate::bus::BusHandle;
use crate::tui::swarm_view::SwarmEvent;

/// Handle a lifecycle event, returning `true` when consumed.
pub(super) fn handle(handle: &BusHandle, event: &SwarmEvent) -> bool {
    match event {
        SwarmEvent::Started { task, .. } => {
            handle.announce_ready(vec![format!("executing:{task}")]);
            handle.send_task_update("swarm", TaskState::Working, Some(task.clone()));
        }
        SwarmEvent::SubTaskUpdate {
            id,
            name,
            status,
            agent_name,
        } => {
            handle.send_task_update(
                id,
                TaskState::Working,
                Some(format!("{name}: {status:?} ({agent_name:?})")),
            );
        }
        SwarmEvent::AgentComplete {
            subtask_id,
            success,
            steps,
        } => {
            handle.send_task_update(subtask_id, state(*success), Some(format!("steps={steps}")));
        }
        SwarmEvent::AgentError { subtask_id, error } => {
            handle.send_task_update(subtask_id, TaskState::Failed, Some(error.clone()));
        }
        SwarmEvent::Complete { success, .. } => {
            handle.send_task_update("swarm", state(*success), None);
        }
        SwarmEvent::Error(err) => {
            handle.send_task_update("swarm", TaskState::Failed, Some(err.clone()));
        }
        _ => return false,
    }
    true
}

fn state(success: bool) -> TaskState {
    if success {
        TaskState::Completed
    } else {
        TaskState::Failed
    }
}
