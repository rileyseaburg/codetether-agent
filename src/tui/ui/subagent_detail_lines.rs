//! Bind selection and native sessions to shared child-detail routing.
use crate::tui::app::state::AppState;
use ratatui::text::Line;
#[path = "subagent_detail_tool/host.rs"]
mod registry;
pub(super) fn lines(state: &AppState) -> Vec<Line<'static>> {
    codetether_agent_dashboard::selection::lines(
        state.active_spawned_agent.as_deref(),
        &state.spawned_agents,
        state.session_id.as_deref(),
        &registry::Registry,
    )
}

#[cfg(test)]
#[path = "subagent_detail_lines_tests.rs"]
mod tests;
