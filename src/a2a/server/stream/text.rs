//! Collect textual prompt parts without executing the request.
use super::*;
pub(super) fn text(params: &MessageSendParams) -> String {
    params
        .message
        .parts
        .iter()
        .filter_map(|p| match p {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
