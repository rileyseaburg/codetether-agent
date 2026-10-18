//! Native selection between shared dashboard and detail-panel rendering.
use crate::tui::app::state::AppState;
use ratatui::{Frame, layout::Rect};
/// Render managed children and swarm workers in one dashboard.
pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    if state.subagent_detail_mode {
        super::subagent_detail_view::render(f, area, state);
        return;
    }
    codetether_agent_dashboard::render::dashboard(f, area, super::subagent_lines::lines(state));
}
