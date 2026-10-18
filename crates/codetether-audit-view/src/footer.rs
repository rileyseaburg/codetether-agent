//! Audit view footer rendering.
use crate::AuditViewState;
use ratatui::{Frame, layout::Rect, style::Stylize, text::Line, widgets::Paragraph};

pub(crate) fn render_footer(f: &mut Frame, _state: &AuditViewState, area: Rect) {
    let hint = Line::from(vec![
        "↑/↓".bold(),
        " select · ".dim(),
        "f".bold(),
        " toggle filter · ".dim(),
        "Esc".bold(),
        " chat".dim(),
    ]);
    f.render_widget(Paragraph::new(hint), area);
}
