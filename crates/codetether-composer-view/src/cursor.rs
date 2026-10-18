//! Input-area cursor placement.
//!
//! [`place_cursor`] positions the terminal cursor inside the input rectangle
//! accounting for multi-line text and Unicode wide characters.

use ratatui::{Frame, layout::Rect};
use unicode_width::UnicodeWidthStr;

use crate::ComposerView;

/// Place the cursor using Unicode character position and display widths.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `view` - Input text and cursor character index.
/// * `area` - Input rectangle including its borders.
///
/// # Returns
/// Sets a cursor position clamped to the panel.
///
/// # Examples
/// ```no_run
/// # fn draw(f: &mut ratatui::Frame) {
/// let view = codetether_composer_view::ComposerView { text: "hello", cursor: 5, processing: false, command_mode: false };
/// let area = f.area();
/// codetether_composer_view::place_cursor(f, &view, area);
/// # }
/// ```
pub fn place_cursor(f: &mut Frame, view: &ComposerView<'_>, area: Rect) {
    let text = view.text;
    let cursor_byte = text
        .char_indices()
        .nth(view.cursor)
        .map(|(i, _)| i)
        .unwrap_or(text.len());
    let before = &text[..cursor_byte];
    let row = before.matches('\n').count();
    let last_nl = before.rfind('\n').map(|i| i + 1).unwrap_or(0);
    let col = before[last_nl..].width();
    let x = area
        .x
        .saturating_add(1)
        .saturating_add(col as u16)
        .min(area.x.saturating_add(area.width.saturating_sub(2)));
    let y = area
        .y
        .saturating_add(1)
        .saturating_add(row as u16)
        .min(area.y.saturating_add(area.height.saturating_sub(2)));
    f.set_cursor_position((x, y));
}
