//! Terminal-state transitions for one inbound task.

use dashmap::DashMap;

use crate::a2a::types::{Message, Task, TaskState};

use super::super::server_settle_build as build;

pub(super) fn complete(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    text: String,
) -> Message {
    let message = build::agent_message(task_id, context_id, text);
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Completed;
        task.status.message = Some(message.clone());
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
        task.artifacts.push(build::response_artifact(&message));
        task.history.push(message.clone());
    }
    message
}

pub(super) fn fail(
    tasks: &DashMap<String, Task>,
    task_id: &str,
    context_id: Option<&str>,
    error: &anyhow::Error,
) -> Message {
    let message = build::agent_message(task_id, context_id, format!("Error: {error}"));
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Failed;
        task.status.message = Some(message.clone());
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
    message
}
