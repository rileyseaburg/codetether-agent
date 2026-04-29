//! Helper functions for extracting content from internal [`Message`].

use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

/// Collect all text content from a message.
pub(super) fn collect_text(m: &Message) -> String {
    m.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

/// Collect all thinking content from a message.
pub(super) fn collect_thinking(m: &Message) -> String {
    m.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Thinking { text } => Some(text.clone()),
            _ => None,
        })
        .collect()
}

/// Collect tool calls as DeepSeek API JSON values.
pub(super) fn collect_calls(m: &Message) -> Vec<Value> {
    m.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id, name, arguments, ..
            } => Some(json!({
                "id": id,
                "type": "function",
                "function": {"name": name, "arguments": arguments}
            })),
            _ => None,
        })
        .collect()
}
