//! Ralph render header presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render_header(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let (pending, running, passed, failed) = state.status_counts();
    let total = state.stories.len();

    let (title, border_color) = super::header_style::style(state, failed);
    let lines = vec![
        super::run_identity::row(state),
        super::run_counts::row(state, pending, running, passed, failed, total),
    ];

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(Style::default().fg(border_color)),
    );
    f.render_widget(paragraph, area);
}
