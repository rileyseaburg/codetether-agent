//! Stable focus-ring construction across both managed-agent registries.

use crate::tui::app::state::App;

pub(super) fn agent_names(app: &App) -> Vec<String> {
    let mut names = crate::tui::app::state::agent_tree::dfs_order(&app.state.spawned_agents)
        .into_iter()
        .map(|node| node.name)
        .collect::<Vec<_>>();
    let Some(parent) = app.state.session_id.as_deref() else {
        return names;
    };
    let mut tools = crate::tool::agent::bridge::list_agent_tool_agents_for_parent(parent);
    tools.sort_by(|left, right| left.name.cmp(&right.name));
    for tool in tools {
        if !names.contains(&tool.name) {
            names.push(tool.name);
        }
    }
    names
}
