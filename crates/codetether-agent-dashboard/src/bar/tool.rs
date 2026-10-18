//! Agent-tool-spawned agents → agent-bar tab spans (issue #297 Part A).

use super::tab::{AgentTabMeta, agent_tab};
use codetether_agent_tree::SpawnedAgent;
use codetether_agent_view::AgentSnapshot;
use ratatui::text::Span;
use std::collections::HashMap;

/// Push tab spans for agents from the agent-tool store, deduplicating
/// against the TUI `spawned_agents` map by name.
/// # Arguments
/// Receives destination spans, ordered snapshots, local names and selected name.
/// # Returns
/// Appends tool agents whose names are absent from the local registry.
/// # Examples
/// ```
/// let mut spans = vec![];
/// let agents = std::collections::HashMap::<String, codetether_agent_tree::SpawnedAgent<()>>::new();
/// codetether_agent_dashboard::bar::push_tool_agents(&mut spans, &[], &agents, None);
/// assert!(spans.is_empty());
/// ```
pub fn push_tool_agents<S>(
    spans: &mut Vec<Span<'static>>,
    snapshots: &[AgentSnapshot],
    agents: &HashMap<String, SpawnedAgent<S>>,
    active: Option<&str>,
) {
    let tui_names: std::collections::HashSet<&str> = agents.keys().map(String::as_str).collect();
    for snap in snapshots {
        if tui_names.contains(snap.name.as_str()) {
            continue;
        }
        spans.push(Span::raw(" "));
        let identity = snap.origin.is_remote().then(|| snap.origin.lineage());
        spans.push(agent_tab(AgentTabMeta {
            name: &snap.name,
            model_id: snap.origin.model_id(),
            session_id: None,
            identity: identity.as_deref(),
            indent: snap.depth + 1,
            selected: active == Some(snap.name.as_str()),
            processing: snap.is_processing,
        }));
    }
}
