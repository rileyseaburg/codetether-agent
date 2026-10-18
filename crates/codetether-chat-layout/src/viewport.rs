//! Visible-range selection for the pre-wrapped chat line buffer.

use ratatui::text::Line;

/// Returns the rows visible inside a bordered message panel.
///
/// # Arguments
/// * `lines` - Already-wrapped content lines.
/// * `scroll` - First visible row.
/// * `panel_height` - Panel height including borders.
///
/// # Returns
/// The bounded slice within the panel interior.
///
/// # Examples
/// ```
/// let lines = vec![ratatui::text::Line::from("one")];
/// assert_eq!(codetether_chat_layout::visible_lines(&lines, 0, 3).len(), 1);
/// ```
pub fn visible_lines<'a>(
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
