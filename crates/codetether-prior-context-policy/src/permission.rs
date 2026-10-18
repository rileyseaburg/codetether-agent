//! Explicit prior-context permission vocabulary.

const PREFIXES: &[&str] = &[
    "please ",
    "can you ",
    "could you ",
    "you can ",
    "you may ",
    "i allow you to ",
    "feel free to ",
    "go ahead and ",
    "why not ",
    "it is ok to ",
    "its ok to ",
];
/// Return whether a clause explicitly restores persistent access.
pub(super) fn persistent(text: &str) -> bool {
    super::reenable::matches(text, PREFIXES)
        || text.starts_with("allow ")
        || text.starts_with("i allow ")
        || text.starts_with("i permit ")
        || text.contains("restriction is lifted")
        || text.contains(" back on")
        || (once(text)
            && ["again", "from now on"]
                .iter()
                .any(|term| text.contains(term)))
}

/// Return whether a clause explicitly permits access for the current turn.
pub(super) fn once(text: &str) -> bool {
    super::permission_action::matches(text)
        || PREFIXES.iter().any(|prefix| {
            text.strip_prefix(prefix)
                .is_some_and(super::permission_action::matches)
        })
        || text.starts_with("i allow ")
        || text.starts_with("i permit ")
}
