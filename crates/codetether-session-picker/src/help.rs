//! Session picker keybinding help rendering.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub(super) fn render(f: &mut Frame, bus_badge: Span<'static>, area: Rect) {
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
        bus_badge,
    ]);
    f.render_widget(Paragraph::new(line), area);
}
