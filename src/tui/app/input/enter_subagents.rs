//! Native entry point for shared dashboard Enter-key navigation.
use crate::tui::app::state::App;
#[cfg(test)]
use crate::tui::models::ViewMode;
#[path = "enter_subagents_host.rs"]
mod host;
pub(super) fn dispatch(app: &mut App) {
    codetether_agent_dashboard::interaction::enter(app);
}
#[cfg(test)]
#[path = "enter_swarm_tests.rs"]
mod swarm_tests;
#[cfg(test)]
#[path = "enter_subagents_tests.rs"]
mod tests;
