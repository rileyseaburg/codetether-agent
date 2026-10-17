//! collect reasoning conversion.
use super::*;

pub(super) fn collect_reasoning(msg: &Message) -> Value {
    let s: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Thinking { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("");
    if s.is_empty() {
        Value::Null
    } else {
        serde_json::json!(s)
    }
}
