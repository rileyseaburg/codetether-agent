//! Audit view header rendering.
use crate::AuditViewState;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::Paragraph,
};

pub(crate) fn render_header(f: &mut Frame, state: &AuditViewState, area: Rect) {
    let line = Line::from(vec![
        "Audit".bold(),
        " · ".dim(),
        Span::styled(
            state.filter.label(),
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        " · ".dim(),
        format!("{} entries", state.entries.len()).into(),
    ]);
    f.render_widget(Paragraph::new(line), area);
}
