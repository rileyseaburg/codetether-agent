//! Visible session rows and selection-state rendering.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Borders, List, ListState},
};

use crate::tui::app::state::App;

pub(super) fn render(f: &mut Frame, app: &App, area: Rect) {
    let filtered = app.state.filtered_sessions();
    let rows = area.height.saturating_sub(2) as usize;
    let selected = app
        .state
        .selected_session
        .min(filtered.len().saturating_sub(1));
    let range = super::window::bounds(selected, filtered.len(), rows);
    let items = super::item::items(app, &filtered, range.clone());
    let title = if filtered.is_empty() {
        " Available Sessions ".to_string()
    } else {
        format!(" Available Sessions ({}/{}) ", selected + 1, filtered.len())
    };
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title(title))
        .highlight_style(Style::default().bg(Color::DarkGray).fg(Color::Cyan).bold())
        .highlight_symbol("▶ ");
    let mut state = ListState::default();
    if !range.is_empty() {
        state.select(Some(selected - range.start));
    }
    f.render_stateful_widget(list, area, &mut state);
}
