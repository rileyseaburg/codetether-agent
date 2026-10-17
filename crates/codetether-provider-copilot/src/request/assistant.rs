//! assistant role encoding.
use crate::*;
pub(super) fn encode(msg: &Message) -> Value {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    let tool_calls: Vec<Value> = msg
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
                "function": {
                    "name": name,
                    "arguments": arguments
                }
            })),
            _ => None,
        })
        .collect();

    if tool_calls.is_empty() {
        json!({ "role": "assistant", "content": text })
    } else {
        json!({
            "role": "assistant",
            "content": if text.is_empty() { Value::Null } else { json!(text) },
            "tool_calls": tool_calls
        })
    }
}
