//! Symbol picker render.
mod area;
mod input;
mod results;
mod status;

use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Clear,
};

use super::SymbolSearchState;

/// Renders the centered symbol picker, query, results and status.
/// # Arguments
/// * `frame` - Target terminal frame.
/// * `state` - Picker state, including list selection.
/// * `area` - Available terminal rectangle.
/// # Returns
/// Unit after rendering the popup and synchronizing list selection.
/// # Examples
/// ```no_run
/// # fn example(frame: &mut ratatui::Frame, area: ratatui::layout::Rect) {
/// let mut state = codetether_symbol_search::SymbolSearchState::new();
/// codetether_symbol_search::render_symbol_search(frame, &mut state, area);
/// # }
/// ```
pub fn render_symbol_search(frame: &mut Frame, state: &mut SymbolSearchState, area: Rect) {
    let popup = area::centered(area, 80, 60);
    frame.render_widget(Clear, popup);
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(3),
            Constraint::Length(1),
        ])
        .split(popup);
    input::render(frame, state, rows[0]);
    results::render(frame, state, rows[1]);
    status::render(frame, state, rows[2]);
}
