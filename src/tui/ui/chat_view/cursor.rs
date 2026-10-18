//! Application adapter for composer cursor placement.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Place the cursor using the current Unicode character index.
///
/// # Arguments
/// * `f` - Destination frame.
/// * `app` - Current text and cursor.
/// * `area` - Input rectangle.
///
/// # Returns
/// Draws the component without modifying the input state.
///
/// # Examples
/// ```no_run
/// use codetether_agent::tui::ui::chat_view::cursor::place_cursor;
/// # fn draw(f: &mut ratatui::Frame, app: &codetether_agent::tui::app::state::App) {
/// let area = f.area();
/// place_cursor(f, app, area);
/// # }
/// ```
pub fn place_cursor(f: &mut Frame, app: &App, area: Rect) {
    codetether_composer_view::place_cursor(f, &super::composer_view::projection(app), area);
}
