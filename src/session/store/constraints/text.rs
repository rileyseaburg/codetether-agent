//! Bounded text projection shared by stored and newly evicted constraints.
use crate::provider::{ContentPart, Message};
pub(in crate::session::store) fn excerpt(message: &Message) -> Option<String> {
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
