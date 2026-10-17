//! Shared forage text rules.
/// Trim a goal and truncate it with an ellipsis when over the limit.
///
/// # Arguments
/// * `goal` — Goal text.
/// * `max_chars` — Character limit before the suffix.
/// # Returns
/// Trimmed and optionally truncated text.
/// # Examples
/// ```
/// assert_eq!(codetether_forage_core::truncate_goal(" abc ", 8), "abc");
/// ```
pub fn truncate_goal(goal: &str, max_chars: usize) -> String {
    let trimmed = goal.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_string();
    }
    truncate_chars_with_suffix(trimmed, max_chars, "...")
}

/// Bound text by characters and append the supplied suffix when truncated.
///
/// # Arguments
/// * `value` — Text to bound.
/// * `max_chars` — Maximum retained characters.
/// * `suffix` — Marker added after truncation.
/// # Returns
/// Original or truncated text.
/// # Examples
/// ```
/// assert_eq!(codetether_forage_core::truncate_chars_with_suffix("abc", 2, "!"), "ab!");
/// ```
pub fn truncate_chars_with_suffix(value: &str, max_chars: usize, suffix: &str) -> String {
    if value.chars().count() <= max_chars {
        return value.to_string();
    }

    let mut out = value.chars().take(max_chars).collect::<String>();
    out.push_str(suffix);
    out
}
