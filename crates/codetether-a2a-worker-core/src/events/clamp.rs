//! Structured worker event clamp.

/// Returns `text` bounded to `limit` bytes on a character boundary.
pub(super) fn clamp(text: &str, limit: usize) -> (String, bool) {
    if text.len() <= limit {
        return (text.to_string(), false);
    }
    (
        codetether_util::truncate::truncate_bytes_safe(text, limit).to_string(),
        true,
    )
}
