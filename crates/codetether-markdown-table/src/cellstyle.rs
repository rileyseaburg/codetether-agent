//! Diff-aware and status-aware coloring for table data cells.

use ratatui::style::{Color, Style};

/// Choose a cell style based on its content.
///
/// Cells that are purely an added count (`+165`) render green; purely
/// removed (`-127`) render red. Mixed (`+165/-127`), emoji status cells,
/// and anything else keep `base` so they color themselves.
pub(crate) fn cell_style(text: &str, base: Style) -> Style {
    let t = text.trim();
    if t.is_empty() {
        return base;
    }
    let has_add = t.contains('+');
    let has_del = t.contains('-');
    let numeric = t
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '+' | '-' | '/' | ',' | ' '));
    if !numeric {
        return base;
    }
    match (has_add, has_del) {
        (true, false) => base.fg(Color::Green),
        (false, true) => base.fg(Color::Red),
        _ => base,
    }
}
