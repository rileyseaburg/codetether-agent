//! Ralph render story detail presentation.
use crate::RalphViewState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render_story_detail(f: &mut Frame, state: &RalphViewState, area: Rect) {
    let story = match state.selected_story() {
        Some(s) => s,
        None => {
            let p = Paragraph::new("No story selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Story Detail "),
            );
            f.render_widget(p, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(6), // Story info header
            Constraint::Min(1),    // Content
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    super::detail_header::render(f, story, chunks[0]);
    // --- Content: tool history, messages, output, error ---
    super::detail_content::render(f, story, state.detail_scroll, chunks[1]);
    super::detail_hints::render(f, chunks[2]);
}
