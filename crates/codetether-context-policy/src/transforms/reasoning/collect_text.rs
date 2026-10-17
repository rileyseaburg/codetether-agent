//! collect text conversion.
use super::*;

pub(super) fn collect_text(msg: &Message) -> String {
    msg.content
        .iter()
        .filter_map(|p| match p {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
