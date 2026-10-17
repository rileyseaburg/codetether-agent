//! Extract newline-separated text for message roles.
use crate::*;
pub(super) fn content(msg: &Message) -> String {
    let content = msg
        .content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    content
}
