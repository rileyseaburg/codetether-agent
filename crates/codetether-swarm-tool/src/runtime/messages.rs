//! Initial worker event and user prompt construction.
use super::WorkerContext;
use crate::task_input::TaskInput;
use codetether_swarm_core::SwarmEvent;
use tokio::sync::mpsc;
pub(super) fn started(
    events: &mpsc::Sender<SwarmEvent>,
    context: &WorkerContext,
    task: &TaskInput,
) {
    let _ = events.try_send(SwarmEvent::AgentStarted {
        subtask_id: context.task_id.clone(),
        agent_name: format!("agent-{}", context.task_id),
        specialty: task.specialty.as_deref().unwrap_or("Generalist").into(),
    });
}
pub(super) fn user(task: &TaskInput) -> String {
    let user = format!(
        "Task: {}\nSpecialty: {}\n\nInstruction: {}",
        task.name,
        task.specialty.as_deref().unwrap_or("Generalist execution"),
        task.instruction
    );

    user
}
