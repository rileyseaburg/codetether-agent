//! Registry fields used to assemble the agent header.
use codetether_agent_tree::SpawnedAgent;
use codetether_agent_view::AgentSnapshot;
use codetether_swarm_view::SubTaskInfo;
use std::collections::HashMap;
/// Durable session identity displayed on a local-agent tab.
/// # Implementors
/// Native sessions expose their existing identifier.
pub trait SessionId {
    /// Borrow the session identifier.
    fn session_id(&self) -> &str;
}
/// Native snapshots and selection state for the unified header.
/// # Implementors
/// Frontends provide local agents, tool snapshots and swarm worker state.
pub trait Host {
    /// Native local-agent session type.
    type Session: SessionId;
    /// Borrow the local agent registry.
    fn agents(&self) -> &HashMap<String, SpawnedAgent<Self::Session>>;
    /// Load tool-agent snapshots in their existing order and parent scope.
    fn tool_agents(&self) -> Vec<AgentSnapshot>;
    /// Borrow current swarm worker summaries.
    fn subtasks(&self) -> &[SubTaskInfo];
    /// Borrow the selected agent name, if any.
    fn active(&self) -> Option<&str>;
    /// Resolve the current model label.
    fn model(&self) -> Option<String>;
    /// Borrow the main session identifier.
    fn session_id(&self) -> Option<&str>;
    /// Whether the main conversation is processing.
    fn processing(&self) -> bool;
}
