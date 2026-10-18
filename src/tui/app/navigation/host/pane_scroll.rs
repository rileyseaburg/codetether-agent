//! Native pane scroll operations requested by navigation policy.
use crate::tui::app::state::App;
use codetether_tui_navigation::Action;
pub(super) fn apply(app: &mut App, action: &Action) -> bool {
    match action {
        Action::ToolUp(value) => {
            app.state.tool_viewport.scroll_tool_preview_up(*value);
        }
        Action::ToolDown(value) => {
            app.state.tool_viewport.scroll_tool_preview_down(*value);
        }
        Action::RalphDetailUp(value) => {
            app.state.ralph.detail_scroll_up(*value);
        }
        Action::RalphDetailDown(value) => {
            app.state.ralph.detail_scroll_down(*value);
        }
        Action::BusDetailUp(value) => {
            app.state.bus_log.detail_scroll_up(*value);
        }
        Action::BusDetailDown(value) => {
            app.state.bus_log.detail_scroll_down(*value);
        }
        Action::SwarmDetailUp(value) => {
            app.state.swarm.detail_scroll_up(*value);
        }
        Action::SwarmDetailDown(value) => {
            app.state.swarm.detail_scroll_down(*value);
        }
        Action::SubagentScroll(value) => {
            app.state.agent_tree.detail_scroll = *value;
        }
        _ => return false,
    }
    true
}
