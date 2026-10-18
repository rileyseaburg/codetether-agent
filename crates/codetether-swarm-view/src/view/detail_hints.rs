//! Swarm detail hints presentation.
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
pub(super) fn render(f: &mut Frame, area: Rect) {
    // --- Key hints bar ---
    let hints = Paragraph::new(Line::from(vec![
        Span::styled(" Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back  "),
        Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
        Span::raw(": Scroll  "),
        Span::styled("↑/↓", Style::default().fg(Color::Yellow)),
        Span::raw(": Prev/Next agent"),
    ]));
    f.render_widget(hints, area);
}
