//! Visible-range selection for the pre-wrapped chat line buffer.

use ratatui::text::Line;

/// Returns the rows visible inside a bordered message panel.
pub(crate) fn visible_lines<'a>(
    lines: &'a [Line<'static>],
    scroll: u16,
    panel_height: u16,
) -> &'a [Line<'static>] {
    let count = panel_height.saturating_sub(2) as usize;
    let start = usize::from(scroll).min(lines.len());
    let end = start.saturating_add(count).min(lines.len());
    &lines[start..end]
}

#[cfg(test)]
#[path = "viewport_tests.rs"]
mod tests;
