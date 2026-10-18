//! Native registry and view operations for shared dashboard selection.
use crate::tui::{app::state::App, models::ViewMode};
use codetether_swarm_view::SwarmViewState;
impl codetether_agent_dashboard::interaction::Host for App {
    fn selected(&self) -> Option<&str> {
        self.state.active_spawned_agent.as_deref()
    }
    fn clear_selected(&mut self) {
        self.state.active_spawned_agent = None;
    }
    fn local_contains(&self, name: &str) -> bool {
        self.state.spawned_agents.contains_key(name)
    }
    fn owned_tool_contains(&self, name: &str) -> bool {
        self.state.session_id.as_deref().is_some_and(|parent| {
            crate::tool::agent::bridge::find_agent_tool_agent_for_parent(name, parent).is_some()
        })
    }
    fn swarm(&self) -> &SwarmViewState {
        &self.state.swarm
    }
    fn swarm_mut(&mut self) -> &mut SwarmViewState {
        &mut self.state.swarm
    }
    fn show_swarm(&mut self) {
        self.state.set_view_mode(ViewMode::Swarm);
    }
    fn open_local_detail(&mut self) {
        self.state.subagent_detail_mode = true;
        self.state.subagent_detail_scroll = 0;
    }
    fn cycle_focus(&mut self) {
        crate::tui::app::navigation::cycle_agent_focus(self);
    }
    fn set_status(&mut self, status: String) {
        self.state.status = status;
    }
}
