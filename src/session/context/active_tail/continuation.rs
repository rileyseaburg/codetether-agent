//! Detection of short user continuation messages.

use crate::provider::{ContentPart, Message};

const MAX_CHARS: usize = 80;

/// Return whether a message is a continuation rather than a new task.
pub(super) fn is_short(message: &Message) -> bool {
    let text = message
        .content
        .iter()
        .find_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .unwrap_or("");
    let normalized = text.trim().to_ascii_lowercase();
    normalized.len() <= MAX_CHARS
        && matches!(
            normalized.trim_matches(|c: char| c.is_ascii_punctuation()),
            "continue"
                | "status"
                | "resume"
                | "go"
                | "go on"
                | "keep going"
                | "hey"
                | "ok"
                | "okay"
                | "yes"
                | "y"
        )
}
