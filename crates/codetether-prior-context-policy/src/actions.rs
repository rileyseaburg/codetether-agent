//! Access-action vocabulary for prior-context directives.

const WORDS: &[&str] = &[
    "use", "call", "recall", "inspect", "access", "search", "browse", "read", "load", "restore",
    "check", "open", "allow", "enable", "permit", "turn",
];
const PHRASES: &[&str] = &["look at", "look through"];

/// Return whether a normalized clause names an access action.
pub(super) fn mentioned(text: &str) -> bool {
    text.split_whitespace().any(|word| WORDS.contains(&word))
        || PHRASES.iter().any(|phrase| text.contains(phrase))
}
