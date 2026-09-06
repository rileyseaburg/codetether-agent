//! Bedrock system-text extraction.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn append_system(msg: &Message, system_parts: &mut Vec<Value>) {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    if !text.trim().is_empty() {
        system_parts.push(json!({"text": text}));
    }
}
