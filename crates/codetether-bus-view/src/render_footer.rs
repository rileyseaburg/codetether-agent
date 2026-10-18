//! Footer help renderer for the bus log view.

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};

pub(super) fn render_footer(f: &mut Frame, area: Rect) {
    let footer = Paragraph::new(Line::from(vec![
        key("↑↓"),
        Span::raw(": nav  "),
        key("Enter"),
        Span::raw(": detail/apply  "),
        key("/"),
        Span::raw(": filter  "),
        key("Backspace"),
        Span::raw(": edit  "),
        key("c"),
        Span::raw(": clear  "),
        key("Esc"),
        Span::raw(": back/close filter"),
    ]));
    f.render_widget(footer, area);
}

fn key(text: &'static str) -> Span<'static> {
    Span::styled(text, Style::default().fg(Color::Yellow))
}
