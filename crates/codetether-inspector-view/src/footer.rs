//! Inspector footer model and navigation hint.
use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::Line,
    widgets::Paragraph,
};

pub(super) fn render_footer(f: &mut Frame, state: &crate::InspectorView<'_>, area: Rect) {
    let footer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(area);

    let help = Paragraph::new(Line::styled(
        format!(
            " Esc: back to chat │ Model: {} ",
            state.last_completion_model.as_deref().unwrap_or("auto")
        ),
        Style::default().fg(Color::DarkGray),
    ));
    f.render_widget(help, footer[1]);
}
