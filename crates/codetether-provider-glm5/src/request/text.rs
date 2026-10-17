//! Ordinary role text encoding.
use crate::*;
pub(super) fn encode(msg: &Message, role: &str) -> Value {
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    json!({"role": role, "content": text})
}
