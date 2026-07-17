//! Stable focus-ring construction across all child-agent registries.

use crate::tool::agent::bridge::AgentSnapshot;
use crate::tui::app::state::App;

pub(super) fn agent_names(app: &App) -> Vec<String> {
    let tools = app
        .state
        .session_id
        .as_deref()
        .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
        .unwrap_or_default();
    assemble(app, tools)
}

fn assemble(app: &App, mut tools: Vec<AgentSnapshot>) -> Vec<String> {
    let mut names = crate::tui::app::state::agent_tree::dfs_order(&app.state.spawned_agents)
        .into_iter()
        .map(|node| node.name)
        .collect::<Vec<_>>();
    tools.sort_by(|left, right| left.name.cmp(&right.name));
    for tool in tools {
        if !names.contains(&tool.name) {
            names.push(tool.name);
        }
    }
    for task in &app.state.swarm.subtasks {
        if let Some(name) = task.agent_name.as_ref()
            && !names.contains(name)
        {
            names.push(name.clone());
        }
    }
    names
}

#[cfg(test)]
#[path = "agent_focus_swarm_tests.rs"]
mod tests;
