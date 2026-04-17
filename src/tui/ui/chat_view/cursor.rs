//! Input-area cursor placement.
//!
//! [`place_cursor`] positions the terminal cursor inside the input rectangle
//! accounting for multi-line text and Unicode wide characters.

use ratatui::{Frame, layout::Rect};
use unicode_width::UnicodeWidthStr;

use crate::tui::app::state::App;

/// Set the terminal cursor inside the input `area`.
///
/// Accounts for newlines (row offset) and wide characters (column offset)
/// so the cursor visually lands at [`AppState::input_cursor`].
///
/// # Examples
///
/// ```rust,no_run
/// // Requires a live terminal backend; cannot run in doc tests.
/// use codetether_agent::tui::ui::chat_view::cursor::place_cursor;
/// # fn demo(f: &mut ratatui::Frame, app: &codetether_agent::tui::app::state::App) {
/// let area = ratatui::layout::Rect::new(0, 0, 40, 3);
/// place_cursor(f, app, area);
/// # }
/// ```
pub fn place_cursor(f: &mut Frame, app: &App, area: Rect) {
    let text = &app.state.input;
    let cursor_byte = text
        .char_indices()
        .nth(app.state.input_cursor)
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
