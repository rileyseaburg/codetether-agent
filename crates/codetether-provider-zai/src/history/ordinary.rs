//! Ordinary system, developer, and user message conversion.
use super::HistoryFormat;
use crate::{ContentPart, Message, Role};
use serde_json::{Value, json};
pub(super) fn convert(message: &Message, format: HistoryFormat) -> Value {
    let parts: Vec<&str> = message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    let text = parts.join(match format {
        HistoryFormat::SingleReply => "\n",
        HistoryFormat::ExpandedReplies => "",
    });
    let role = match message.role {
        Role::System | Role::Developer => "system",
        Role::User => "user",
        Role::Assistant => "assistant",
        Role::Tool => "tool",
    };
    json!({"role": role, "content": text})
}
