//! Resolve selected agents before replacing the main chat transcript.
use crate::tool_detail;
use codetether_agent_tree::SpawnedAgent;
use codetether_session_store::Snapshot;
use ratatui::text::Line;
use std::collections::HashMap;

/// Build detail rows only when the selected agent still exists.
///
/// Managed sessions take precedence over the parent-scoped tool registry.
/// # Arguments
/// Receives the selected name, managed sessions, current parent identity and
/// the registry used to resolve tool-spawned or remote agents.
/// # Returns
/// Detail rows for a known selection, or `None` when absent or stale.
/// # Examples
/// ```
/// # fn example<S: codetether_session_store::Snapshot, T: codetether_agent_dashboard::tool_detail::Host>(
/// # agents: &std::collections::HashMap<String, codetether_agent_tree::SpawnedAgent<S>>, registry: &T) {
/// assert!(codetether_agent_dashboard::selection::known_lines(None, agents, None, registry).is_none());
/// # }
/// ```
pub fn known_lines<S: Snapshot, T: tool_detail::Host>(
    selected: Option<&str>,
    agents: &HashMap<String, SpawnedAgent<S>>,
    parent: Option<&str>,
    registry: &T,
) -> Option<Vec<Line<'static>>> {
    let name = selected?;
    if !agents.contains_key(name)
        && !parent.is_some_and(|parent| registry.find(name, parent).is_some())
    {
        return None;
    }
    Some(super::lines(selected, agents, parent, registry))
}
