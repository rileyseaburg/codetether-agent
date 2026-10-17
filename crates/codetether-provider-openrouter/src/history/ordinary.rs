//! ordinary message shape.
use crate::*;
pub(super) fn convert(msg: &Message, role: &str) -> Value {
    // System or User message
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
