//! Normalize line endings and clean rendered terminal text.
use crate::{has_tui_artifacts, strip_tui_artifacts};

/// Normalise line endings and strip TUI render artifacts.
///
/// Used by the paste handler so mouse-selected TUI text arrives clean.
/// # Arguments
/// * `text` - Copied terminal selection or pasted plain text.
/// # Returns
/// Cleaned text preserving ordinary input and paragraph boundaries.
/// # Examples
/// ```
/// assert_eq!(codetether_terminal_text::normalize_paste("a\r\nb"), "a\nb");
/// ```
pub fn normalize_paste(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    if has_tui_artifacts(&normalized) {
        strip_tui_artifacts(&normalized)
    } else {
        normalized
    }
}
