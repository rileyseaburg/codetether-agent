//! Scrollable managed-agent transcript renderer.

use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, layout::Rect};

use crate::tui::app::state::AppState;

pub(super) fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let name = state.active_spawned_agent.as_deref().unwrap_or("unknown");
    let block = Block::default()
        .title(format!(" Agent detail: @{name} "))
        .borders(Borders::ALL);
    let paragraph = Paragraph::new(super::subagent_detail_lines::lines(state))
        .block(block)
        .wrap(Wrap { trim: false })
        .scroll((state.subagent_detail_scroll, 0));
    f.render_widget(paragraph, area);
}
