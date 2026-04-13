//! Text helpers for agent tool output.
//!
//! This module contains small string utilities used by agent actions.
//! Keeping them here avoids mixing formatting helpers into provider or
//! session management modules.
//!
//! # Examples
//!
//! ```ignore
//! assert_eq!(truncate_preview("abcdef", 3), "abc...");
//! ```

/// Truncates a string to a character-safe preview with an ellipsis.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(truncate_preview("abcdef", 3), "abc...");
/// ```
pub(super) fn truncate_preview(input: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let preview: String = input.chars().take(max_chars).collect();
    if input.chars().nth(max_chars).is_some() {
        return format!("{preview}...");
    }
    preview
}
