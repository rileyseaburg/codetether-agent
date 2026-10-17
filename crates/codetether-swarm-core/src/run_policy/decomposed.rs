//! TUI projection of decomposed subtasks.

use crate::{SubTask, SubTaskStatus};
use crate::{SubTaskInfo, SwarmEvent};

/// Project newly decomposed tasks into a shared lifecycle event.
///
/// # Arguments
/// * `subtasks` - Tasks to display.
/// * `max_steps` - Per-task step budget.
///
/// # Returns
/// A decomposed event with initial pending statuses.
///
/// # Examples
/// ```
/// let event = codetether_swarm_core::run_policy::decomposed_event(&[], 10);
/// assert!(matches!(event, codetether_swarm_core::SwarmEvent::Decomposed { .. }));
/// ```
pub fn build(subtasks: &[SubTask], max_steps: usize) -> SwarmEvent {
    SwarmEvent::Decomposed {
        subtasks: subtasks
            .iter()
            .map(|task| SubTaskInfo {
                id: task.id.clone(),
                name: task.name.clone(),
                status: SubTaskStatus::Pending,
                stage: task.stage,
                dependencies: task.dependencies.clone(),
                agent_name: task.specialty.clone(),
                current_tool: None,
                steps: 0,
                max_steps,
                tool_call_history: Vec::new(),
                messages: Vec::new(),
                output: None,
                error: None,
                started_at: None,
                elapsed_secs: None,
            })
            .collect(),
    }
}
