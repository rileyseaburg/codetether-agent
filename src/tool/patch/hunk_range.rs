//! Decode a unified-diff range, including implicit and zero line counts.

pub(super) fn parse(range: &str, prefix: char) -> Option<(usize, usize)> {
    let range = range.strip_prefix(prefix)?;
    let (start, count) = range.split_once(',').unwrap_or((range, "1"));
    Some((start.parse().ok()?, count.parse().ok()?))
}
