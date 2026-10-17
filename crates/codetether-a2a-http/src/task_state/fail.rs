//! fail task operation.
use super::*;

pub fn fail(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    error: &anyhow::Error,
) -> Message {
    let message = agent_message(task_id, context_id, format!("Error: {error}"));
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Failed;
        task.status.message = Some(message.clone());
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
    message
}
