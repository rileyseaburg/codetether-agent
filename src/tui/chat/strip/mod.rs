//! Strip TUI box-drawing render artifacts from text.
//!
//! When a user mouse-selects text from the chat panel the terminal
//! emulator copies the raw rendered screen, including ratatui
//! [`Block`](ratatui::widgets::Block) borders (`│`, `┌`, `└`, `─`, …)
//! and trailing whitespace padding. This module cleans that up so
//! pasted-back content is the underlying plain text.

mod strip_line;
#[cfg(test)]
mod tests;

use strip_line::{is_border_line, strip_line};

/// Remove TUI box-drawing render artifacts from `text`.
///
/// Drops pure border lines and strips leading/trailing border characters
/// from content lines. Multiple consecutive blank lines are collapsed to one.
pub fn strip_tui_artifacts(text: &str) -> String {
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
            out.push_str(stripped);
            out.push('\n');
            last_blank = false;
        }
    }
    if out.ends_with('\n') {
        out.pop();
    }
    out
}

/// Normalise line endings and strip TUI render artifacts.
///
/// Used by the paste handler so mouse-selected TUI text arrives clean.
pub fn normalize_paste(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    if has_tui_artifacts(&normalized) {
        strip_tui_artifacts(&normalized)
    } else {
        normalized
    }
}

/// Return `true` when `text` appears to contain TUI render artifacts.
pub fn has_tui_artifacts(text: &str) -> bool {
    text.lines()
        .any(|l| l.contains('│') || l.contains('┌') || l.contains('└'))
}
