//! Focused detail selection from managed sessions and a parent-scoped registry.
mod known;
use crate::{Child, detail, tool_detail};
use codetether_agent_tree::SpawnedAgent;
use codetether_session_store::Snapshot;
pub use known::known_lines;
use ratatui::text::Line;
use std::collections::HashMap;

struct Selection<'a, S, T> {
    selected: Option<&'a str>,
    agents: &'a HashMap<String, SpawnedAgent<S>>,
    parent: Option<&'a str>,
    registry: &'a T,
}

impl<S: Snapshot, T: tool_detail::Host> detail::Host for Selection<'_, S, T> {
    fn selected(&self) -> Option<&str> {
        self.selected
    }
    fn managed(&self, name: &str) -> Option<Vec<Line<'static>>> {
        self.agents
            .get(name)
            .map(|agent| crate::managed_detail::lines(&agent.display(), agent.session.messages()))
    }
    fn tool(&self, name: &str) -> Vec<Line<'static>> {
        tool_detail::lines(self.registry, self.parent, name)
    }
}

/// Build focused child details, preferring a managed session with that name.
/// # Arguments
/// Receives the selected name, managed sessions, parent identity and the
/// registry bridge used for parent-scoped tool-agent reads.
/// # Returns
/// Managed or tool-agent detail rows, or the existing empty-selection hint.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_session_store::Snapshot, T: codetether_agent_dashboard::tool_detail::Host>(
/// # agents: &std::collections::HashMap<String, codetether_agent_tree::SpawnedAgent<S>>, registry: &T) {
/// let rows = codetether_agent_dashboard::selection::lines(None, agents, None, registry);
/// assert!(!rows.is_empty());
/// # }
/// ```
pub fn lines<S: Snapshot, T: tool_detail::Host>(
    selected: Option<&str>,
    agents: &HashMap<String, SpawnedAgent<S>>,
    parent: Option<&str>,
    registry: &T,
) -> Vec<Line<'static>> {
    detail::lines(&Selection {
        selected,
        agents,
        parent,
        registry,
    })
}
