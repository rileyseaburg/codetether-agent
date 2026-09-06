//! Assistant message → JSON conversion.

use serde_json::{Value, json};

use crate::provider::{ContentPart, Message};

pub(super) fn assistant_json(msg: &Message, text: String) -> Value {
    let mut obj = json!({ "role": "assistant" });
    if !text.is_empty() {
        obj["content"] = json!(text);
    }
    let calls: Vec<Value> = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(json!({
                "id": id,
                "type": "function",
                "function": { "name": name, "arguments": arguments },
            })),
            _ => None,
        })
        .collect();
    if !calls.is_empty() {
        obj["tool_calls"] = Value::Array(calls);
    }
    obj
}
