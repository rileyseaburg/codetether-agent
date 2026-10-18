//! Unified agent-dashboard row composition.

use ratatui::text::Line;

use crate::tool::agent::bridge::{AgentSnapshot, list_agent_tool_agents};
use crate::tui::app::state::AppState;

/// Build dashboard rows for managed, tool-spawned, and swarm agents.
pub fn lines(state: &AppState) -> Vec<Line<'static>> {
    let tools = state
        .session_id
        .as_deref()
        .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
        .unwrap_or_else(list_agent_tool_agents);
    compose(state, &tools)
}

fn compose(state: &AppState, tool_agents: &[AgentSnapshot]) -> Vec<Line<'static>> {
    codetether_agent_dashboard::dashboard::compose(
        &codetether_agent_dashboard::dashboard::Dashboard {
            agents: &state.spawned_agents,
            swarm: &state.swarm,
            active_tasks: state.active_tasks.count(),
            selected: state.active_spawned_agent.as_deref(),
        },
        tool_agents,
    )
}

#[cfg(test)]
#[path = "subagent_lines_tests.rs"]
mod tests;
