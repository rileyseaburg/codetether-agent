//! Classify inbound A2A messages as mesh introductions.
//!
//! Intros are presence announcements, not work. Detecting them lets the
//! server answer with a canned reply instead of burning an LLM call and
//! persisting a junk session per intro (see `docs/a2a-spawn.md`).

use crate::a2a::types::{Message, Part};

/// Metadata key senders set to mark a message as a mesh introduction.
pub const INTRO_METADATA_KEY: &str = "codetether.intro";

/// The intro text template prefix used by [`super::send::send_intro`].
const INTRO_TEXT_PREFIX: &str = "Hello from ";
const INTRO_TEXT_SUFFIX: &str = "available for A2A collaboration.";

/// Whether `message` is a mesh introduction.
///
/// Checks the explicit metadata tag first (new senders), then falls back
/// to matching the exact legacy intro template so old peers still in the
/// wild are short-circuited too.
pub fn is_intro(message: &Message) -> bool {
    if message
        .metadata
        .get(INTRO_METADATA_KEY)
        .and_then(serde_json::Value::as_bool)
        == Some(true)
    {
        return true;
    }
    matches_legacy_template(message)
}

fn matches_legacy_template(message: &Message) -> bool {
    let text: String = message
        .parts
        .iter()
        .filter_map(|p| match p {
            Part::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");
    let trimmed = text.trim();
    trimmed.starts_with(INTRO_TEXT_PREFIX) && trimmed.ends_with(INTRO_TEXT_SUFFIX)
}
