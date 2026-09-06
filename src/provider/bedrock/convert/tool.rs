//! Bedrock tool-result blocks and their associated image siblings.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(super) fn append_tool(msg: &Message, api_messages: &mut Vec<Value>) {
    let mut parts: Vec<Value> = Vec::new();
    for part in &msg.content {
        match part {
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => {
                let text = if content.trim().is_empty() {
                    "(empty tool result)"
                } else {
                    content
                };
                parts.push(json!({"toolResult": {
                    "toolUseId": tool_call_id,
                    "content": [{"text": text}],
                    "status": "success"
                }}));
            }
            ContentPart::Image { url, mime_type } => {
                if let Some(result) = parts.last_mut()
                    && let Some(blocks) = result["toolResult"]["content"].as_array_mut()
                {
                    blocks.push(super::image::block(url, mime_type.as_deref()));
                }
            }
            _ => {}
        }
    }
    super::merge::append(api_messages, "user", parts);
}
