//! Agent-header state with frontend-selected tool and model sources.
use super::{Host, SessionId};
use codetether_agent_tree::{AgentTreeState, SpawnedAgent};
use codetether_agent_view::AgentSnapshot;
use codetether_swarm_view::SubTaskInfo;
use std::collections::HashMap;

/// Borrowed state and lazy data sources for the unified agent header.
///
/// Tool and model callbacks preserve the frontend's lookup scope and are called
/// only when rendering or visibility checks request those values.
///
/// # Examples
/// ```
/// use codetether_agent_dashboard::bar::{Header, Host, SessionId};
/// fn example<S: SessionId>(tree: &codetether_agent_tree::AgentTreeState<S>) {
///     let header = Header {
///         tree, subtasks: &[], session_id: None, processing: false,
///         tools: Vec::<codetether_agent_view::AgentSnapshot>::new,
///         model: || None,
///     };
///     assert!(!header.processing());
/// }
/// ```
pub struct Header<'a, S, T, M> {
    /// Local spawned agents and the current selection.
    pub tree: &'a AgentTreeState<S>,
    /// Current swarm task summaries.
    pub subtasks: &'a [SubTaskInfo],
    /// Main conversation identifier, if assigned.
    pub session_id: Option<&'a str>,
    /// Whether the main conversation is processing.
    pub processing: bool,
    /// Fetch tool-agent snapshots in frontend-defined scope and order.
    pub tools: T,
    /// Resolve the current model label when requested.
    pub model: M,
}

impl<S, T, M> Host for Header<'_, S, T, M>
where
    S: SessionId,
    T: Fn() -> Vec<AgentSnapshot>,
    M: Fn() -> Option<String>,
{
    type Session = S;
    fn agents(&self) -> &HashMap<String, SpawnedAgent<S>> {
        &self.tree.spawned
    }
    fn tool_agents(&self) -> Vec<AgentSnapshot> {
        (self.tools)()
    }
    fn subtasks(&self) -> &[SubTaskInfo] {
        self.subtasks
    }
    fn active(&self) -> Option<&str> {
        self.tree.active.as_deref()
    }
    fn model(&self) -> Option<String> {
        (self.model)()
    }
    fn session_id(&self) -> Option<&str> {
        self.session_id
    }
    fn processing(&self) -> bool {
        self.processing
    }
}
