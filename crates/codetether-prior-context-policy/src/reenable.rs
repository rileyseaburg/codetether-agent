//! Persistent re-enable phrase matching.

/// Match explicit enable/re-enable phrases after an optional permission prefix.
pub(super) fn matches(text: &str, prefixes: &[&str]) -> bool {
    ["re enable ", "reenable ", "enable "].iter().any(|phrase| {
        text.starts_with(phrase)
            || prefixes.iter().any(|prefix| {
                text.strip_prefix(prefix)
                    .is_some_and(|tail| tail.trim_start_matches("now ").starts_with(phrase))
            })
    })
}
