//! Shared utility helpers used across modules.

/// Truncates `s` to at most `max_bytes` bytes while respecting UTF-8 char boundaries.
///
/// If `s` is shorter than or equal to `max_bytes` bytes it is returned unchanged.
/// Otherwise the longest prefix that fits within `max_bytes` bytes **and** ends on
/// a valid UTF-8 boundary is returned, so the result is always a valid `&str`.
///
/// This is safe to use for building diagnostic messages from arbitrary API
/// response bodies, which may contain multi-byte characters like em-dashes.
pub fn truncate_bytes_safe(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ascii_shorter_than_limit() {
        assert_eq!(truncate_bytes_safe("hello", 10), "hello");
    }

    #[test]
    fn ascii_exactly_at_limit() {
        assert_eq!(truncate_bytes_safe("hello", 5), "hello");
    }

    #[test]
    fn ascii_truncated() {
        assert_eq!(truncate_bytes_safe("hello world", 5), "hello");
    }

    #[test]
    fn multibyte_on_boundary() {
        // em dash U+2014 → 3 bytes (0xE2 0x80 0x94); sits at bytes 3..6
        let s = "abc\u{2014}def";
        assert_eq!(truncate_bytes_safe(s, 6), "abc\u{2014}");
    }

    #[test]
    fn multibyte_cut_inside_char_does_not_panic() {
        // Reproduces the crash: byte 300 inside em dash (bytes 299..302)
        let prefix = "a".repeat(299);
        let s = format!("{prefix}\u{2014}trailing");
        // byte index 300 sits inside the em dash
        let result = truncate_bytes_safe(&s, 300);
        assert_eq!(result, prefix.as_str());
    }

    #[test]
    fn zero_limit_returns_empty() {
        assert_eq!(truncate_bytes_safe("hello", 0), "");
    }
}
