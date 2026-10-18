//! Project native registry state into the shared terminal view.
use crate::tui::app::state::App;
use ratatui::{Frame, layout::Rect};
/// Render registered agent cards using the current selection and scroll state.
pub fn render_protocol_registry(f: &mut Frame, app: &mut App, area: Rect) {
    let view = codetether_protocol_view::RegistryView {
        registered_agents: &app.state.worker_bridge_registered_agents,
        worker_id: &app.state.worker_id,
        selected: app.state.protocol_selected,
        scroll: app.state.protocol_scroll,
    };
    codetether_protocol_view::render_protocol_registry(f, &view, area);
}
