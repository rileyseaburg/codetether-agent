//! Observer events for subtasks blocked by failed prerequisites.

use crate::SwarmEvent;
use crate::{SubTask, SubTaskResult, SubTaskStatus};

/// Emit lifecycle events for blocked tasks in result order.
///
/// # Arguments
/// * `tasks` - Tasks used to resolve names and IDs.
/// * `blocked` - Results blocked by failed dependencies.
/// * `emit` - Host callback for each constructed event.
///
/// # Returns
/// Nothing; emits update, optional error and completion events per matching task.
///
/// # Examples
/// ```
/// let mut events = Vec::new();
/// codetether_swarm_core::run_policy::report_blocked(&[], &[], |event| events.push(event));
/// assert!(events.is_empty());
/// ```
pub fn report(tasks: &[SubTask], blocked: &[SubTaskResult], mut emit: impl FnMut(SwarmEvent)) {
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
