//! Native state projection into the workspace snapshot panel.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
pub fn render(f: &mut Frame, app: &App, area: Rect) {
    codetether_workspace_view::render(f, &app.state.workspace, area);
}
