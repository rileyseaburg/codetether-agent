//! Reflow rendered rows after removing terminal box borders.
use crate::{
    has_tui_artifacts,
    strip_line::{is_border_line, strip_line},
};

/// Remove TUI box-drawing render artifacts from `text`.
///
/// Drops pure border lines and strips leading/trailing border characters
/// from content lines. Adjacent visual rows are reflowed with spaces while
/// blank rows preserve paragraph breaks.
/// # Arguments
/// * `text` - Copied terminal selection or pasted plain text.
/// # Returns
/// Cleaned text preserving ordinary input and paragraph boundaries.
/// # Examples
/// ```
/// assert_eq!(codetether_terminal_text::strip_tui_artifacts("│ hello │"), "hello");
/// ```
pub fn strip_tui_artifacts(text: &str) -> String {
    if !has_tui_artifacts(text) {
        return text.to_string();
    }
    let mut out = String::with_capacity(text.len());
    let mut last_blank = false;
    for line in text.lines() {
        if is_border_line(line) {
            continue;
        }
        let stripped = strip_line(line);
        if stripped.is_empty() {
            if !last_blank && !out.is_empty() {
                out.push('\n');
            }
            last_blank = true;
        } else {
            if !out.is_empty() {
                out.push(if last_blank { '\n' } else { ' ' });
            }
            out.push_str(stripped);
            last_blank = false;
        }
    }
    out
}
