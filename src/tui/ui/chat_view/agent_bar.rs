//! Native header rendering and visibility entry points.
mod host;
#[path = "agent_bar_presence.rs"]
pub(super) mod presence;
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Render the shared main/local/tool/swarm agent header.
pub fn render_agent_bar(f: &mut Frame, app: &App, area: Rect) {
    codetether_agent_dashboard::bar::render(f, &host::header(app), area);
}
