//! Dashboard section for managed and agent-tool children.

use std::collections::HashSet;

use ratatui::style::Stylize;
use ratatui::text::Line;

use crate::{Child, dashboard::Dashboard};
use codetether_agent_view::AgentSnapshot;

/// Append all non-swarm children, deduplicated by name.
pub(crate) fn append<C: Child>(
    rows: &mut Vec<Line<'static>>,
    state: &Dashboard<'_, C>,
    tool_agents: &[AgentSnapshot],
) {
    if state.agents.is_empty() && tool_agents.is_empty() {
        return;
    }
    rows.push(Line::from("Managed children".cyan().bold()));
    let mut agents = state
        .agents
        .values()
        .map(Child::display)
        .collect::<Vec<_>>();
    agents.sort_by_key(|agent| (agent.parent, agent.depth, agent.name));
    rows.extend(
        agents
            .into_iter()
            .map(|agent| crate::managed::line(&agent, state.selected == Some(agent.name))),
    );

    let managed = state
        .agents
        .keys()
        .map(String::as_str)
        .collect::<HashSet<_>>();
    let mut tools = tool_agents
        .iter()
        .filter(|agent| !managed.contains(agent.name.as_str()))
        .collect::<Vec<_>>();
    tools.sort_by_key(|agent| (agent.origin.parent(), agent.depth, &agent.name));
    rows.extend(
        tools
            .into_iter()
            .map(|agent| crate::tool_row::line(agent, state.selected == Some(agent.name.as_str()))),
    );
    rows.push(Line::from(""));
}
