//! Character-count limited previews with an ellipsis when truncated.
/// Limit text to a number of Unicode scalar values and mark omitted content.
///
/// # Arguments
/// * `text` - Source text.
/// * `max_chars` - Maximum retained characters before the ellipsis.
///
/// # Returns
/// The original text or a truncated prefix with an ellipsis.
///
/// # Examples
/// ```
/// assert_eq!(codetether_util::preview_chars::truncate_preview("hello", 3), "hel…");
/// ```
pub fn truncate_preview(text: &str, max_chars: usize) -> String {
    let mut chars = text.chars();
    let preview: String = chars.by_ref().take(max_chars).collect();
    if chars.next().is_some() {
        format!("{preview}…")
    } else {
        preview
    }
}
