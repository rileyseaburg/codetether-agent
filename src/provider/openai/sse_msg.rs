//! Message → JSON conversion for the raw-SSE reasoning request body.

use serde_json::{Value, json};

use crate::provider::{ContentPart, Message, Role};

use super::sse_msg_assist::{assistant_json, tool_json};

pub(super) fn messages_json(messages: &[Message]) -> Vec<Value> {
    messages.iter().map(message_json).collect()
}

fn message_json(msg: &Message) -> Value {
    let text = joined_text(msg);
    match msg.role {
        Role::System => json!({ "role": "system", "content": text }),
        Role::User => json!({ "role": "user", "content": text }),
        Role::Assistant => assistant_json(msg, text),
        Role::Tool => tool_json(msg),
    }
}

fn joined_text(msg: &Message) -> String {
    msg.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
