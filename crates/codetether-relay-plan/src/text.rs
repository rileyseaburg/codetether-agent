//! Character-bounded relay display text.

/// Keep at most the requested number of characters before an ellipsis.
/// # Arguments
/// * `value` - Text to display.
/// * `max_chars` - Characters retained before any appended ellipsis.
/// # Returns
/// The original text when short enough; empty for zero; otherwise a prefix plus ellipsis.
/// # Examples
/// ```
/// assert_eq!(codetether_relay_plan::truncate_with_ellipsis("abcd", 2), "ab...");
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
