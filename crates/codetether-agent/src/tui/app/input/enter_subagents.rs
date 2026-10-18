//! Frontend type and callback selection for dashboard Enter handling.
#[cfg(test)]
use crate::tui::models::ViewMode;
codetether_agent_dashboard::declare_frontend_dashboard_input!(
    crate::tui::app::state::App,
    crate::tui::models::ViewMode,
    codetether_swarm_view::SwarmViewState;
    crate::tool::agent::bridge::find_agent_tool_agent_for_parent,
    crate::tui::app::navigation::cycle_agent_focus,
);
#[cfg(test)]
#[path = "enter_swarm_tests.rs"]
mod swarm_tests;
#[cfg(test)]
#[path = "enter_subagents_tests.rs"]
mod tests;
