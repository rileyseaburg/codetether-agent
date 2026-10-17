//! Bound diagnostic text by Unicode character count.

/// Limit text by Unicode character count, adding an ellipsis when truncated.
///
/// # Arguments
/// * `value` - Text to limit.
/// * `max_chars` - Number of retained characters, excluding the ellipsis.
/// # Returns
/// Original or shortened text; an empty string for a zero limit.
/// # Examples
/// ```
/// assert_eq!(codetether_rlm::legacy_support::truncate_with_ellipsis("abcd", 2), "ab...");
/// ```
pub fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}
