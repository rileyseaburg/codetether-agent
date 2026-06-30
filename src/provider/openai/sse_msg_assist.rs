//! Assistant/tool message → JSON conversion helpers.

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

pub(super) fn tool_json(msg: &Message) -> Value {
    for part in &msg.content {
        if let ContentPart::ToolResult {
            tool_call_id,
            content,
        } = part
        {
            return json!({
                "role": "tool",
                "tool_call_id": tool_call_id,
                "content": content,
            });
        }
    }
    json!({ "role": "tool", "content": "" })
}
