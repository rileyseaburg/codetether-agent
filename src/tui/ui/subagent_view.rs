//! Native binding for shared agent dashboard and detail-panel selection.
mod host;
use crate::tui::app::state::AppState;
use ratatui::{Frame, layout::Rect};
/// Render managed children and swarm workers in one dashboard.
pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    codetether_agent_dashboard::view::render(f, area, &host::Panel(state));
}
