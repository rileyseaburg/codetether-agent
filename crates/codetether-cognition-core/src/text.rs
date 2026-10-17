//! Text trimming and counting helpers for stored cognition payloads.

/// Truncate `input` to `max_chars` characters, appending an ellipsis.
///
/// # Examples
///
/// ```rust
/// # use codetether_cognition_core::text::trim_for_storage;
/// assert_eq!(trim_for_storage("  hello  ", 32), "hello");
/// assert_eq!(trim_for_storage("abcdef", 3), "abc...");
/// ```
///
/// # Arguments
/// * `input` - Text to trim.
///
/// # Returns
/// The result of the policy described above.
pub fn trim_for_storage(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }
    let mut trimmed: String = input.chars().take(max_chars).collect();
    trimmed.push_str("...");
    trimmed.trim().to_string()
}

/// Estimate how many discrete facts a thought contains, by sentence count.
///
/// # Examples
///
/// ```rust
/// # use codetether_cognition_core::text::estimate_fact_count;
/// assert_eq!(estimate_fact_count("no punctuation"), 1);
/// assert_eq!(estimate_fact_count("One. Two. Three."), 3);
/// ```
///
/// # Arguments
/// * `text` - Thought text whose sentences are counted.
///
/// # Returns
/// The result of the policy described above.
pub fn estimate_fact_count(text: &str) -> usize {
    let sentences =
        text.matches('.').count() + text.matches('!').count() + text.matches('?').count();
    sentences.clamp(1, 12)
}
