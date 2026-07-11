//! Sub-agent dashboard renderer.

use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, layout::Rect};

use crate::tui::app::state::AppState;

use super::subagent_lines;

/// Render managed children and swarm workers in one dashboard.
pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let block = Block::default()
        .title(" Agents: managed children · tool agents · swarm workers ")
        .borders(Borders::ALL);
    f.render_widget(
        Paragraph::new(subagent_lines::lines(state)).block(block),
        area,
    );
}
