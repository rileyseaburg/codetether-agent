//! Native binding for shared agent dashboard and detail-panel selection.
use crate::tui::app::state::AppState;
use ratatui::{Frame, layout::Rect};
/// Render managed children and swarm workers in one dashboard.
pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let panel = codetether_agent_dashboard::panel::Panel {
        tree: &state.agent_tree,
        dashboard: || super::subagent_lines::lines(state),
        detail: || super::subagent_detail_lines::lines(state),
    };
    codetether_agent_dashboard::view::render(f, area, &panel);
}
