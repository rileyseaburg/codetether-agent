//! Unified agent-dashboard row composition.

use ratatui::style::Stylize;
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
    let tool_count = tool_agents
        .iter()
        .filter(|agent| !state.spawned_agents.contains_key(&agent.name))
        .count();
    let managed_count = state.spawned_agents.len() + tool_count;
    let swarm_count = state.swarm.subtasks.len();
    let mut rows = vec![Line::from(
        format!(
            "active tasks: {} · managed: {managed_count} · swarm: {swarm_count}",
            state.active_tasks.count()
        )
        .green(),
    )];
    rows.push(Line::from(
        "Tab/↑↓: child · Enter: transcript · /swarm: worker details".dim(),
    ));
    rows.push(Line::from(""));
    super::subagent_managed_lines::append(&mut rows, state, tool_agents);
    super::subagent_swarm_lines::append(&mut rows, &state.swarm);
    if managed_count == 0 && swarm_count == 0 {
        rows.push(Line::from(
            "No agents yet. Use /spawn, the agent tool, or /swarm <task>.".dim(),
        ));
    }
    rows
}

#[cfg(test)]
#[path = "subagent_lines_tests.rs"]
mod tests;
