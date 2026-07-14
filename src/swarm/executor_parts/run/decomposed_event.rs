//! TUI projection of decomposed subtasks.

use crate::swarm::{SubTask, SubTaskStatus};
use crate::tui::swarm_view::{SubTaskInfo, SwarmEvent};

pub(super) fn build(subtasks: &[SubTask], max_steps: usize) -> SwarmEvent {
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
