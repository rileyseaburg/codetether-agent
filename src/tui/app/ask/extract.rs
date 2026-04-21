//! Extract plain text from a provider [`Message`].
//!
//! Concatenates all [`ContentPart::Text`] parts and drops any tool
//! calls or other non-text parts. Used by `/ask` to render the model's
//! reply as a single system message in the TUI.

use crate::provider::{ContentPart, Message};

/// Concatenate every [`ContentPart::Text`] in `message`.
///
/// Returns an empty string when `message` has no text parts. Non-text
/// parts (tool calls, tool results, images) are skipped.
pub(super) fn extract_text(message: &Message) -> String {
    message
        .content
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("")
}
