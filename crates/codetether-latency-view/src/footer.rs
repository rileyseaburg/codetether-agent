//! Inspector navigation and processing status footer.
use crate::LatencyView;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
};
pub(crate) fn render(f: &mut Frame, area: Rect, view: &LatencyView<'_>) {
    let footer = Paragraph::new(Line::from(vec![
        Span::styled(
            " LATENCY ",
            Style::default().fg(Color::Black).bg(Color::Cyan),
        ),
        Span::raw(" | "),
        Span::styled("Esc", Style::default().fg(Color::Yellow)),
        Span::raw(": Back | "),
        Span::styled("/chat", Style::default().fg(Color::Yellow)),
        Span::raw(": Return | "),
        Span::styled(
            view.status.clone(),
            Style::default().fg(if view.processing {
                Color::Yellow
            } else {
                Color::Green
            }),
        ),
    ]));
    f.render_widget(footer, area);
}
