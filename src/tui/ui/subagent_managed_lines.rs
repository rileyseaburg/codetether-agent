//! Dashboard section for managed and agent-tool children.

use std::collections::HashSet;

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::tool::agent::bridge::AgentSnapshot;
use crate::tui::app::state::AppState;

/// Append all non-swarm children, deduplicated by name.
pub fn append(rows: &mut Vec<Line<'static>>, state: &AppState, tool_agents: &[AgentSnapshot]) {
    if state.spawned_agents.is_empty() && tool_agents.is_empty() {
        return;
    }
    rows.push(Line::from("Managed children".cyan().bold()));
    let mut agents = state.spawned_agents.values().collect::<Vec<_>>();
    agents.sort_by_key(|agent| (&agent.parent, agent.depth, &agent.name));
    rows.extend(agents.into_iter().map(super::subagent_row::line));

    let managed = state
        .spawned_agents
        .keys()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut tools = tool_agents
        .iter()
        .filter(|agent| !managed.contains(agent.name.as_str()))
        .collect::<Vec<_>>();
    tools.sort_by_key(|agent| (&agent.parent, agent.depth, &agent.name));
    rows.extend(tools.into_iter().map(super::subagent_tool_row::line));
    rows.push(Line::from(""));
}
