//! Live chat window selection.

use codetether_provider_core::Message;
use codetether_view_retention::CHAT_RETAINED_MAX_ITEMS;

/// Select the retained tail while preserving chronological order.
///
/// # Arguments
/// * `history` — Chronological provider messages.
///
/// # Returns
/// A borrowed iterator limited by the shared chat retention cap.
///
/// # Examples
/// ```
/// assert_eq!(codetether_chat_transcript::sync::recent(&[]).count(), 0);
/// ```
pub fn recent(history: &[Message]) -> impl DoubleEndedIterator<Item = &Message> {
    history.iter().rev().take(CHAT_RETAINED_MAX_ITEMS).rev()
}
