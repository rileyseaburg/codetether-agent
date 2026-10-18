//! Stable focus-ring construction across all child-agent registries.

use crate::tool::agent::bridge::AgentSnapshot;
use crate::tui::app::state::App;

pub(in crate::tui::app::navigation) fn agent_names(app: &App) -> Vec<String> {
    let tools = app
        .state
        .session_id
        .as_deref()
        .map(crate::tool::agent::bridge::list_agent_tool_agents_for_parent)
        .unwrap_or_default();
    assemble(app, tools)
}
fn assemble(app: &App, tools: Vec<AgentSnapshot>) -> Vec<String> {
    codetether_tui_navigation::focus::assemble(
        crate::tui::app::state::agent_tree::dfs_order(&app.state.agent_tree.spawned)
            .into_iter()
            .map(|node| node.name)
            .collect(),
        tools.into_iter().map(|tool| tool.name).collect(),
        app.state
            .swarm
            .subtasks
            .iter()
            .map(|task| task.agent_name.clone())
            .collect(),
    )
}

#[cfg(test)]
#[path = "agent_focus_swarm_tests.rs"]
mod tests;
