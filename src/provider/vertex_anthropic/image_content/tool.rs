//! Vertex tool images remain inside their preceding result, without new filtering.
use crate::provider::{ContentPart, Message};
use serde_json::{Value, json};

pub(in crate::provider::vertex_anthropic) fn parts(msg: &Message) -> Vec<Value> {
    let mut results: Vec<Value> = Vec::new();
    for part in &msg.content {
        match part {
            ContentPart::ToolResult {
                tool_call_id,
                content,
            } => {
                results.push(json!({"type": "tool_result", "tool_use_id": tool_call_id,
                    "content": content}));
            }
            ContentPart::Image { url, mime_type } => {
                if let Some(result) = results.last_mut() {
                    if let Some(text) = result["content"].as_str() {
                        let blocks = if text.is_empty() {
                            Vec::new()
                        } else {
                            vec![json!({"type": "text", "text": text})]
                        };
                        result["content"] = Value::Array(blocks);
                    }
                    if let Some(blocks) = result["content"].as_array_mut() {
                        blocks.push(super::image_block(url, mime_type.as_deref()));
                    }
                }
            }
            _ => {}
        }
    }
    results
}
