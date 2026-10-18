//! Swarm render agent detail presentation.
use crate::SwarmViewState;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::widgets::{Block, Borders, Paragraph};
pub(super) fn render_agent_detail(f: &mut Frame, state: &SwarmViewState, area: Rect) {
    let task = match state.selected_subtask() {
        Some(t) => t,
        None => {
            let p = Paragraph::new("No subtask selected").block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Agent Detail "),
            );
            f.render_widget(p, area);
            return;
        }
    };

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5), // Agent info header
            Constraint::Min(1),    // Content (tool calls + messages + output)
            Constraint::Length(1), // Key hints
        ])
        .split(area);

    super::detail_header::render(f, task, chunks[0]);
    // --- Content area: tool history, messages, output ---
    super::detail_content::render(f, task, state.detail_scroll, chunks[1]);
    super::detail_hints::render(f, chunks[2]);
}
