//! Swarm render header presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render_header(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let (pending, running, completed, failed) = state.status_counts();
    let total = state.subtasks.len();

    let title = super::header_title::title(state);
    let status_line = super::header_line::line(state, pending, running, completed, failed, total);
    let paragraph = Paragraph::new(status_line).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(if state.complete {
                if state.error.is_some() {
                    Color::Red
                } else {
                    Color::Green
                }
            } else {
                Color::Cyan
            })),
    );

    f.render_widget(paragraph, area);
}
