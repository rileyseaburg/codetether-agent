//! TUI `/spawn` agents → agent-bar tab spans (issue #297 Part A).

use super::agent_tab::{AgentTabMeta, agent_tab};
use crate::tui::app::state::App;
use crate::tui::app::state::agent_tree::dfs_order;
use ratatui::text::Span;

/// Push tab spans for agents from the TUI `spawned_agents` map.
pub fn push_tui_agents(spans: &mut Vec<Span<'static>>, app: &App, active: Option<&str>) {
    for node in dfs_order(&app.state.spawned_agents) {
        if let Some(agent) = app.state.spawned_agents.get(&node.name) {
            spans.push(Span::raw(" "));
            spans.push(agent_tab(AgentTabMeta {
                name: &node.name,
                model_id: agent.model_id.as_deref(),
                session_id: Some(&agent.session.id),
                indent: node.depth + 1,
                selected: active == Some(node.name.as_str()),
                processing: agent.is_processing,
            }));
        }
    }
}
