//! Prior-context target vocabulary.

pub(super) const PHRASES: &[&str] = &[
    "session recall",
    "session history",
    "session data",
    "session transcript",
    "prior session",
    "previous session",
    "past session",
    "earlier session",
    "chat history",
    "conversation history",
    "project memory",
    "prior context",
    "previous context",
];
const WORDS: &[&str] = &["session", "sessions", "memory", "scope", "history"];

/// Detect either a specific prior-context phrase or a conservative denial target.
pub(super) fn mentioned(text: &str) -> bool {
    specific(text) || text.split_whitespace().any(|word| WORDS.contains(&word))
}

/// Detect an unambiguous prior-context phrase suitable for opt-in decisions.
pub(super) fn specific(text: &str) -> bool {
    PHRASES.iter().any(|target| text.contains(target))
}
