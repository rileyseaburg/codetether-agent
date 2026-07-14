//! Agent-tool-spawned agents → agent-bar tab spans (issue #297 Part A).

use super::agent_tab::{AgentTabMeta, agent_tab};
use crate::tool::agent::bridge::AgentSnapshot;
use crate::tui::app::state::App;
use ratatui::text::Span;

/// Push tab spans for agents from the agent-tool store, deduplicating
/// against the TUI `spawned_agents` map by name.
pub fn push_tool_agents(
    spans: &mut Vec<Span<'static>>,
    snapshots: &[AgentSnapshot],
    app: &App,
    active: Option<&str>,
) {
    let tui_names: std::collections::HashSet<&str> = app
        .state
        .spawned_agents
        .keys()
        .map(String::as_str)
        .collect();
    for snap in snapshots {
        if tui_names.contains(snap.name.as_str()) {
            continue;
        }
        spans.push(Span::raw(" "));
        spans.push(agent_tab(AgentTabMeta {
            name: &snap.name,
            model_id: snap.model_id.as_deref(),
            session_id: None,
            indent: snap.depth + 1,
            selected: active == Some(snap.name.as_str()),
            processing: snap.is_processing,
        }));
    }
}
