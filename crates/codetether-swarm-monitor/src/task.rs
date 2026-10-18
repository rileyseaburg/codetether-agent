//! Direct-swarm task identity and initial TUI records.

use codetether_swarm_core::SubTaskStatus;
use codetether_swarm_tool::task_input::TaskInput;
use codetether_swarm_view::{SubTaskInfo, SwarmEvent};
use tokio::sync::mpsc;

pub(super) fn id(task: &TaskInput, index: usize) -> String {
    task.id_for_index(index)
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

/// Publish the initial running state for one direct worker.
/// # Arguments
/// * `events` — Invocation event sender.
/// * `id` — Stable task identifier.
/// * `specialty` — Worker role, defaulting to Generalist.
/// # Returns
/// Attempts delivery without blocking; a full or closed channel drops the event.
/// # Examples
/// ```
/// let mut observer = codetether_swarm_monitor::Observer::begin(&[], 20);
/// codetether_swarm_monitor::started(&observer.sender(), "task-1", None);
/// observer.complete(&[]);
/// ```
pub fn started(events: &mpsc::Sender<SwarmEvent>, id: &str, specialty: Option<&str>) {
    let _ = events.try_send(SwarmEvent::AgentStarted {
        subtask_id: id.into(),
        agent_name: format!("agent-{id}"),
        specialty: specialty.unwrap_or("Generalist").into(),
    });
}
