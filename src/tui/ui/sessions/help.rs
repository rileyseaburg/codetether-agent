//! Session picker keybinding help rendering.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

use crate::tui::app::state::App;
use crate::tui::ui::status_bar::bus_status_badge_span;

pub(super) fn render(f: &mut Frame, app: &App, area: Rect) {
    let key = Style::default().fg(Color::Yellow);
    let line = Line::from(vec![
        Span::styled(
            " SESSION PICKER ",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" "),
        Span::styled("↑↓", key),
        Span::raw(": Nav "),
        Span::styled("Enter", key),
        Span::raw(": Load "),
        Span::styled("Type", key),
        Span::raw(": Filter by id/title "),
        Span::styled("Backspace", key),
        Span::raw(": Edit "),
        Span::styled("Esc", key),
        Span::raw(": Cancel | "),
        bus_status_badge_span(app),
    ]);
    f.render_widget(Paragraph::new(line), area);
}
