//! Pure readers over an inbound `message/send` request.

use dashmap::DashMap;

use crate::a2a::types::{MessageSendParams, Part, Task, TaskState};

pub(super) use super::super::server_settle_build::open_task;

pub(super) fn text_of(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            Part::File { .. } | Part::Data { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// `message/send` blocks by default.
pub(super) fn blocking(params: &MessageSendParams) -> bool {
    params
        .configuration
        .as_ref()
        .and_then(|c| c.blocking)
        .unwrap_or(true)
}

/// The peer's advertised name, when the client supplied one.
pub(super) fn sender_of(params: &MessageSendParams) -> String {
    params
        .message
        .metadata
        .get("codetether.sender")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("remote-a2a")
        .to_string()
}

pub(super) fn fail_empty(tasks: &DashMap<String, Task>, task_id: &str) {
    if let Some(mut task) = tasks.get_mut(task_id) {
        task.status.state = TaskState::Failed;
        task.status.timestamp = Some(chrono::Utc::now().to_rfc3339());
    }
}
