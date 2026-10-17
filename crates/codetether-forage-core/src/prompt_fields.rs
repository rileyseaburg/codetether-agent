//! Shared forage prompt_fields rules.
/// Compact whitespace and bound a prompt field by character count.
///
/// # Arguments
/// * `value` — Original field text.
/// * `max_chars` — Limit, clamped to at least 32 before suffix insertion.
/// # Returns
/// Compact field text with a truncation marker when needed.
/// # Examples
/// ```
/// assert_eq!(codetether_forage_core::normalize_prompt_field("a   b", 32), "a b");
/// ```
pub fn normalize_prompt_field(value: &str, max_chars: usize) -> String {
    let compact = value.split_whitespace().collect::<Vec<_>>().join(" ");
    let normalized_max = max_chars.max(32);
    if compact.chars().count() <= normalized_max {
        return compact;
    }

    let mut truncated = compact.chars().take(normalized_max).collect::<String>();
    truncated.push_str(" ...(truncated)");
    truncated
}
