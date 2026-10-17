//! Assistant text and calls.
use super::calls;
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

    let tool_calls = calls::encode(msg);

    let mut msg_json = json!({
        "role": "assistant",
        "content": if text.is_empty() { Value::Null } else { json!(text) },
    });
    if !tool_calls.is_empty() {
        msg_json["tool_calls"] = json!(tool_calls);
    }
    msg_json
}
