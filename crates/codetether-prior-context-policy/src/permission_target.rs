//! Unambiguous prior-context targets for explicit opt-ins.

const WORDS: &[&str] = &["memory", "sessions", "scope"];

pub(super) fn matches(text: &str) -> bool {
    super::targets::PHRASES
        .iter()
        .chain(WORDS)
        .any(|target| valid_suffix(text, target))
}

fn valid_suffix(text: &str, target: &str) -> bool {
    text.match_indices(target).any(|(index, _)| {
        let before = &text[..index];
        let after = &text[index + target.len()..];
        boundary(before, after) && suffix(after)
    })
}

fn boundary(before: &str, after: &str) -> bool {
    before.chars().last().is_none_or(char::is_whitespace)
        && after.chars().next().is_none_or(char::is_whitespace)
}

fn suffix(after: &str) -> bool {
    after.is_empty()
        || [
            " again",
            " this ",
            " just once",
            " for this ",
            " now",
            " from now on",
            " back on",
            " to ",
        ]
        .iter()
        .any(|allowed| after.starts_with(allowed))
}
