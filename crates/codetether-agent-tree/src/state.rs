//! Spawned-agent registry and its interactive selection state.
use crate::SpawnedAgent;
use std::collections::HashMap;

/// Spawned agents and the frontend state used to inspect their work.
///
/// The application supplies the session type. Empty state does not require a
/// session constructor or a default session value.
///
/// # Examples
/// ```
/// let tree = codetether_agent_tree::AgentTreeState::<()>::default();
/// assert!(tree.spawned.is_empty());
/// assert!(tree.active.is_none());
/// assert!(!tree.detail_mode);
/// assert_eq!(tree.detail_scroll, 0);
/// ```
pub struct AgentTreeState<S> {
    /// Spawned agents indexed by their frontend names.
    pub spawned: HashMap<String, SpawnedAgent<S>>,
    /// Currently selected agent name, if any.
    pub active: Option<String>,
    /// Whether the selected agent's detailed view is open.
    pub detail_mode: bool,
    /// Scroll offset in the selected agent's detailed view.
    pub detail_scroll: u16,
    /// In-progress output indexed by agent name.
    pub streaming_texts: HashMap<String, String>,
}

impl<S> Default for AgentTreeState<S> {
    fn default() -> Self {
        Self {
            spawned: HashMap::new(),
            active: None,
            detail_mode: false,
            detail_scroll: 0,
            streaming_texts: HashMap::new(),
        }
    }
}
