//! Bedrock assistant text and tool-use serialization.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn append_assistant(msg: &Message, api_messages: &mut Vec<Value>) {
    let mut content_parts: Vec<Value> = Vec::new();
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                if !text.trim().is_empty() {
                    content_parts.push(json!({"text": text}));
                }
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => {
                let input: Value =
                    serde_json::from_str(arguments).unwrap_or_else(|_| json!({"raw": arguments}));
                content_parts.push(json!({
                    "toolUse": {
                        "toolUseId": id,
                        "name": name,
                        "input": input
                    }
                }));
            }
            _ => {}
        }
    }
    super::merge::append(api_messages, "assistant", content_parts);
}
