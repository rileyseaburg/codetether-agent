//! Output truncation helpers for shell command results.

/// Largest UTF-8 char boundary at or below `idx`.
fn char_floor(s: &str, mut idx: usize) -> usize {
    while idx > 0 && !s.is_char_boundary(idx) {
        idx -= 1;
    }
    idx
}

/// Cap `text` at `max_bytes`, appending a truncation marker.
pub(super) fn truncate_output(text: &mut String, max_bytes: usize) {
    if text.len() > max_bytes {
        text.truncate(char_floor(text, max_bytes));
        text.push_str("\n... [output truncated]");
    }
}
