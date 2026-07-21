//! Session picker heading and active filter rendering.

use ratatui::{
    Frame,
    layout::Rect,
    style::Stylize,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::tui::app::state::App;

pub(super) fn render(f: &mut Frame, app: &App, area: Rect) {
    let filter = if app.state.session_filter.is_empty() {
        String::new()
    } else {
        format!(" [filter: {}]", app.state.session_filter)
    };
    let content = Line::from(vec![
        Span::raw(" Session picker ").black().on_cyan(),
        Span::raw(" "),
        Span::raw(app.state.status.clone()).dim(),
    ]);
    let title = format!(" Sessions (↑↓ navigate, Enter load, Esc cancel){filter} ");
    let widget = Paragraph::new(content).block(Block::default().borders(Borders::ALL).title(title));
    f.render_widget(widget, area);
}
