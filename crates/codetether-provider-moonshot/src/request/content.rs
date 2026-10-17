//! Extract assistant text and tool-call payloads.
use crate::*;
pub(super) fn text(msg: &Message) -> String {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");

    text
}
pub(super) fn calls(msg: &Message) -> Vec<Value> {
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

    tool_calls
}
