//! Bounded text projection shared by stored and newly evicted constraints.
use codetether_provider_core::{ContentPart, Message};
/// Select up to 120 characters from the first nonempty text part's first line.
/// # Arguments
/// `message` supplies ordered content parts.
/// # Returns
/// A bounded excerpt, or `None` when the message has no nonempty text.
/// # Examples
/// ```rust
/// # fn example(message: &codetether_provider_core::Message) {
/// let text = codetether_session_store::constraints::excerpt(message);
/// assert!(text.is_none_or(|text| text.chars().count() <= 120));
/// # }
/// ```
pub fn excerpt(message: &Message) -> Option<String> {
    message.content.iter().find_map(|part| match part {
        ContentPart::Text { text } if !text.trim().is_empty() => Some(
            text.trim()
                .lines()
                .next()
                .unwrap_or_default()
                .chars()
                .take(120)
                .collect(),
        ),
        _ => None,
    })
}
