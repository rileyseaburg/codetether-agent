//! TUI `/spawn` agents → agent-bar tab spans (issue #297 Part A).

use super::SessionId;
use super::tab::{AgentTabMeta, agent_tab};
use codetether_agent_tree::SpawnedAgent;
use codetether_agent_tree::dfs_order;
use ratatui::text::Span;
use std::collections::HashMap;

/// Push tab spans for agents from the TUI `spawned_agents` map.
/// # Arguments
/// Receives destination spans, the local agent tree and selected name.
/// # Returns
/// Appends parent-before-child tabs, preserving the tree's depth-first order.
/// # Examples
/// ```
/// fn tabs<S: codetether_agent_dashboard::bar::SessionId>(agents: &std::collections::HashMap<String, codetether_agent_tree::SpawnedAgent<S>>) {
///     let mut spans = vec![];
///     codetether_agent_dashboard::bar::push_tui_agents(&mut spans, agents, None);
/// }
/// ```
pub fn push_tui_agents<S: SessionId>(
    spans: &mut Vec<Span<'static>>,
    agents: &HashMap<String, SpawnedAgent<S>>,
    active: Option<&str>,
) {
    for node in dfs_order(agents) {
        if let Some(agent) = agents.get(&node.name) {
            spans.push(Span::raw(" "));
            spans.push(agent_tab(AgentTabMeta {
                name: &node.name,
                model_id: agent.model_id.as_deref(),
                session_id: Some(agent.session.session_id()),
                identity: None,
                indent: node.depth + 1,
                selected: active == Some(node.name.as_str()),
                processing: agent.is_processing,
            }));
        }
    }
}
