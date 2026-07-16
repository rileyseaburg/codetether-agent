//! Direct-swarm task identity and initial TUI records.

use super::super::task_input::TaskInput;
use crate::swarm::SubTaskStatus;
use crate::tui::swarm_view::{SubTaskInfo, SwarmEvent};
use tokio::sync::mpsc;

pub(super) fn id(task: &TaskInput, index: usize) -> String {
    task.id
        .clone()
        .unwrap_or_else(|| format!("task-{}", index + 1))
}

pub(super) fn info(task: &TaskInput, index: usize, max_steps: usize) -> SubTaskInfo {
    SubTaskInfo {
        id: id(task, index),
        name: task.name.clone(),
        status: SubTaskStatus::Pending,
        stage: 0,
        dependencies: Vec::new(),
        agent_name: None,
        current_tool: None,
        steps: 0,
        max_steps,
        tool_call_history: Vec::new(),
        messages: Vec::new(),
        output: None,
        error: None,
        started_at: None,
        elapsed_secs: None,
    }
}

pub(in crate::tool::swarm_execute) fn started(
    events: &mpsc::Sender<SwarmEvent>,
    id: &str,
    specialty: Option<&str>,
) {
    let _ = events.try_send(SwarmEvent::AgentStarted {
        subtask_id: id.into(),
        agent_name: format!("agent-{id}"),
        specialty: specialty.unwrap_or("Generalist").into(),
    });
}
