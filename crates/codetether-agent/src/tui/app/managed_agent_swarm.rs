//! Native swarm-focus adapter and integration test boundary.
use crate::tui::app::state::App;
#[cfg(test)]
use crate::tui::models::ViewMode;
pub(super) fn focus(app: &mut App, query: &str) -> bool {
    codetether_agent_command::focus_swarm(&mut super::host::Native { app, parent: None }, query)
}
#[cfg(test)]
#[path = "managed_agent_swarm_tests.rs"]
mod tests;
