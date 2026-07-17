//! Text and terminal-state extraction from A2A responses.

use crate::a2a::types::{Message, Part, SendMessageResponse, Task, TaskState};

pub(super) fn response(response: &SendMessageResponse) -> (String, bool) {
    match response {
        SendMessageResponse::Message(message) => (message_text(message), false),
        SendMessageResponse::Task(task) => task_result(task),
    }
}

fn task_result(task: &Task) -> (String, bool) {
    let text = task
        .status
        .message
        .as_ref()
        .map(message_text)
        .unwrap_or_else(|| artifact_text(task));
    let failed = matches!(
        task.status.state,
        TaskState::Failed | TaskState::Rejected | TaskState::Cancelled | TaskState::AuthRequired
    );
    (text, failed)
}

fn message_text(message: &Message) -> String {
    parts_text(&message.parts)
}

fn artifact_text(task: &Task) -> String {
    task.artifacts
        .iter()
        .map(|item| parts_text(&item.parts))
        .collect::<Vec<_>>()
        .join("\n")
}

fn parts_text(parts: &[Part]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            Part::Text { text } => Some(text.as_str()),
            Part::File { .. } | Part::Data { .. } => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
