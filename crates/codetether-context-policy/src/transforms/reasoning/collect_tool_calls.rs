//! collect tool calls conversion.
use super::*;

pub(super) fn collect_tool_calls(msg: &Message) -> Value {
    let calls: Vec<Value> = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::ToolCall {
                id,
                name,
                arguments,
                ..
            } => Some(serde_json::json!({
                "id": id, "type": "function", "function": { "name": name, "arguments": arguments }
            })),
            _ => None,
        })
        .collect();
    if calls.is_empty() {
        Value::Null
    } else {
        Value::Array(calls)
    }
}
