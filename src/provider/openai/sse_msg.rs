//! Message → JSON conversion for the raw-SSE reasoning request body.

use serde_json::{Value, json};

use crate::provider::{ContentPart, Message, Role};

use super::chat_messages::{attachments::transport_history, tool::tool_json, user};
use super::sse_msg_assist::assistant_json;

pub(super) fn messages_json(messages: &[Message]) -> Vec<Value> {
    transport_history(messages)
        .iter()
        .flat_map(message_json)
        .collect()
}

fn message_json(msg: &Message) -> Vec<Value> {
    if msg.role == Role::Tool {
        return tool_json(msg);
    }
    let text = joined_text(msg);
    vec![match msg.role {
        Role::System => json!({ "role": "system", "content": text }),
        Role::Developer => json!({ "role": "developer", "content": text }),
        Role::User => json!({ "role": "user", "content": user::content(msg) }),
        Role::Assistant => assistant_json(msg, text),
        Role::Tool => unreachable!("tool replies were expanded above"),
    }]
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
