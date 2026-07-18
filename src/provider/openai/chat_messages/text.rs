//! Plain-text extraction for OpenAI chat messages.

use crate::provider::{ContentPart, Message};

pub(super) fn joined(message: &Message) -> String {
    message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}
