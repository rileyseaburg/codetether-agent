//! assistant content conversion.
use crate::*;
pub(super) fn push(msg: &Message, _system_blocks: &mut Vec<Value>, api_messages: &mut Vec<Value>) {
    let mut content_parts: Vec<Value> = Vec::new();

    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                content_parts.push(json!({
                    "type": "text",
                    "text": text
                }));
            }
            ContentPart::Thinking { text, .. } => {
                content_parts.push(json!({
                    "type": "thinking",
                    "thinking": text
                }));
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
                    "type": "tool_use",
                    "id": id,
                    "name": name,
                    "input": input
                }));
            }
            _ => {}
        }
    }

    if content_parts.is_empty() {
        content_parts.push(json!({"type": "text", "text": " "}));
    }

    api_messages.push(json!({
        "role": "assistant",
        "content": content_parts
    }));
}
