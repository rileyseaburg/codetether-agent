//! UTF-8-safe byte prefix used by content-part rendering.

/// Truncate to `max_len` bytes, respecting char boundaries.
pub(super) fn truncate_bytes(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        return s;
    }
    let mut end = max_len;
    while !s.is_char_boundary(end) && end > 0 {
        end -= 1;
    }
    &s[..end]
}
