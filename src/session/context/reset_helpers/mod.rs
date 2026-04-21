//! Small helpers for the [`DerivePolicy::Reset`](crate::session::derive_policy::DerivePolicy::Reset) pipeline.

use crate::provider::{ContentPart, Message, Role};

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
                "[CONTEXT RESET]\nEverything older than the current user turn was \
                 compressed into the summary below. Recent turns were \
                 intentionally discarded — call `session_recall` if you need \
                 a specific dropped detail.\n\n{summary}"
            ),
        }],
    }
}

#[cfg(test)]
mod tests;
