//! Small helpers for the [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset) pipeline.

use crate::provider::{ContentPart, Message, Role};

const RESET_MARKER_PREFIX: &str = "[CONTEXT RESET]";

/// Index of the last [`Role::User`] message in `messages`, if any.
///
/// Used by [`derive_reset`](super::reset::derive_reset) to preserve
/// the most-recent user turn verbatim while summarising the prefix.
pub(super) fn last_user_index(messages: &[Message]) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .find(|(_, m)| matches!(m.role, Role::User))
        .map(|(i, _)| i)
}

/// Build the synthetic `[CONTEXT RESET]` summary message that replaces
/// the discarded prefix in [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset).
///
/// Kept separate from the derivation body so the exact marker text can
/// be asserted in unit tests without a provider round-trip.
pub(super) fn build_reset_summary_message(summary: &str) -> Message {
    Message {
        role: Role::Assistant,
        content: vec![ContentPart::Text {
            text: format!(
                "{RESET_MARKER_PREFIX}\nEverything older than the current user turn was \
                 compressed into the summary below. Recent turns were \
                 intentionally discarded — call `session_recall` if you need \
                 a specific dropped detail.\n\n{summary}"
            ),
        }],
    }
}

/// Index of the most-recent reset marker in `messages`, if any.
pub(super) fn latest_reset_marker_index(messages: &[Message]) -> Option<usize> {
    messages
        .iter()
        .enumerate()
        .rev()
        .find(|(_, msg)| message_has_reset_marker(msg))
        .map(|(idx, _)| idx)
}

fn message_has_reset_marker(msg: &Message) -> bool {
    msg.content.iter().any(|part| match part {
        ContentPart::Text { text } => text.starts_with(RESET_MARKER_PREFIX),
        ContentPart::ToolResult { content, .. } => content.starts_with(RESET_MARKER_PREFIX),
        _ => false,
    })
}

#[cfg(test)]
mod tests;
