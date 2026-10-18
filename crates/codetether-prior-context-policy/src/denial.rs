//! Explicit prior-context denial vocabulary.

const PHRASES: &[&str] = &[
    "do not ",
    "dont ",
    "never ",
    "cannot ",
    "cant ",
    "must not ",
    "should not ",
    "not use ",
    "not access ",
    "stop ",
    "avoid ",
    "without ",
    "did not ask",
    "didnt ask",
    "not allowed",
    "not permitted",
    "no need",
    "no session",
    "not session",
    "not memory",
    "not prior",
    "not previous",
    "not the session",
    "turn off ",
    "disable ",
    "block ",
    "revoke ",
    "opt out",
    "no scope access",
    "no history access",
    "no prior context access",
];

/// Return whether a normalized clause explicitly denies access.
pub(super) fn matches(text: &str) -> bool {
    PHRASES.iter().any(|phrase| text.contains(phrase))
}
