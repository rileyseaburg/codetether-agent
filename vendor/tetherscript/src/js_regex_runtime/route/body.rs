//! Route regex body and suffix parsing.

pub(super) fn parse(pattern: &str) -> Option<(&str, bool, bool)> {
    let pattern = pattern.strip_prefix('^')?;
    if let Some(body) = pattern.strip_suffix(r"(?:\/(.+)|\/*)$") {
        return Some((body, true, true));
    }
    if let Some(body) = pattern.strip_suffix(r"\/*$") {
        return Some((body, true, false));
    }
    pattern
        .strip_suffix(r"(?:(?=\/|$))")
        .map(|body| (body, false, false))
}
