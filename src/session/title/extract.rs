//! Pure helpers for deriving a title string from session messages.

use super::super::helper::text::truncate_with_ellipsis;
use super::super::types::Session;

/// Derive a truncated title from the first user message, if any.
///
/// Returns `None` when the session has no user message yet.
pub(super) fn title_from_first_user_message(session: &Session) -> Option<String> {
    let msg = session
        .messages
        .iter()
        .find(|m| m.role == crate::provider::Role::User)?;
    let text: String = msg
        .content
        .iter()
        .filter_map(|p| match p {
            crate::provider::ContentPart::Text { text } => Some(text.clone()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join(" ");
    Some(truncate_with_ellipsis(&text, 47))
}
