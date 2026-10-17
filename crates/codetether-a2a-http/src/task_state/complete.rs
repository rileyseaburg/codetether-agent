//! complete task operation.
use super::*;

pub fn complete(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    text: String,
) -> Message {
    let message = agent_message(task_id, context_id, text);
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Completed;
        task.status.message = Some(message.clone());
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        task.artifacts.push(response_artifact(&message));
        task.history.push(message.clone());
    }
    message
}
