//! Project native registry state into the shared terminal view.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Render registered agent cards using the current selection and scroll state.
pub fn render_protocol_registry(f: &mut Frame, app: &mut App, area: Rect) {
    let view = app.state.protocol_registry.view(
        &app.state.worker.worker_bridge_registered_agents,
        &app.state.worker.worker_id,
    );
    codetether_protocol_view::render_protocol_registry(f, &view, area);
}
