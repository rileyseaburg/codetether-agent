//! Assistant text and signed tool-call collection.
use crate::*;
pub(super) fn collect(msg: &Message) -> (Vec<String>, Vec<Value>) {
    let mut text_parts = Vec::new();
    let mut tool_calls = Vec::new();
    for part in &msg.content {
        match part {
            ContentPart::Text { text } => {
                if !text.is_empty() {
                    text_parts.push(text.clone());
                }
            }
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                thought_signature,
            } => {
                let mut tc = json!({
                    "id": id,
                    "type": "function",
                    "function": {
                        "name": name,
                        "arguments": arguments
                    }
                });
                // Include thought signature for Gemini 3.x models
                if let Some(sig) = thought_signature {
                    tc["extra_content"] = json!({
                        "google": {
                            "thought_signature": sig
                        }
                    });
                }
                tool_calls.push(tc);
            }
            _ => {}
        }
    }
    (text_parts, tool_calls)
}
