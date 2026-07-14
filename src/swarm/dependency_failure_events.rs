//! Observer events for subtasks blocked by failed prerequisites.

use crate::swarm::{SubTask, SubTaskResult, SubTaskStatus};
use crate::tui::swarm_view::SwarmEvent;

pub(super) fn report(
    tasks: &[SubTask],
    blocked: &[SubTaskResult],
    mut emit: impl FnMut(SwarmEvent),
) {
    for result in blocked {
        let Some(task) = tasks.iter().find(|task| task.id == result.subtask_id) else {
            continue;
        };
        let agent_name = Some(result.subagent_id.clone());
        emit(SwarmEvent::SubTaskUpdate {
            id: task.id.clone(),
            name: task.name.clone(),
            status: SubTaskStatus::Failed,
            agent_name,
        });
        if let Some(error) = &result.error {
            emit(SwarmEvent::AgentError {
                subtask_id: task.id.clone(),
                error: error.clone(),
            });
        }
        emit(SwarmEvent::AgentComplete {
            subtask_id: task.id.clone(),
            success: false,
            steps: 0,
        });
    }
}

#[cfg(test)]
#[path = "dependency_failure_events_tests.rs"]
mod tests;
