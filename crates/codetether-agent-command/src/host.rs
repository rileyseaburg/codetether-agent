//! State access and operations supplied by a native command frontend.
use crate::roster::LegacyAgent;
use codetether_agent_view::AgentSnapshot;
use codetether_tool_core::ToolResult;
use std::future::Future;
/// Mutable UI state used by agent command decisions.
/// # Implementors
/// Implement with direct state reads/writes and existing UI callbacks.
pub trait State {
    /// Replace the command status text.
    fn status(&mut self, text: String);
    /// Currently focused agent name.
    fn active(&self) -> Option<&str>;
    /// Set or clear the focused agent.
    fn set_active(&mut self, name: Option<String>);
    /// Whether a legacy child with this key exists.
    fn legacy_exists(&self, name: &str) -> bool;
    /// Remove a legacy child, returning whether it existed.
    fn remove_legacy(&mut self, name: &str) -> bool;
    /// Forget streamed legacy output after removal.
    fn clear_legacy_output(&mut self, name: &str);
    /// Ordered optional names of swarm workers.
    fn swarm_names(&self) -> Vec<Option<String>>;
    /// Open the selected worker's detail view and make it active.
    fn select_swarm(&mut self, index: usize);
    /// Invoke the existing agent-dashboard navigation callback.
    fn navigate_agents(&mut self);
    /// Present a tool result using the existing UI convention.
    fn present(&mut self, result: &ToolResult, success: String, failure: &str);
}
/// Parent-scoped agent operations.
/// # Implementors
/// Bind these methods to the existing registry and chat dispatch.
pub trait Registry {
    /// Check ownership under the active parent session.
    fn owned(&self, name: &str) -> bool;
    /// Request removal of an owned managed child.
    fn remove(&mut self, name: &str) -> impl Future<Output = ToolResult>;
    /// Send a message with the existing chat presentation and input handling.
    fn send(&mut self, name: &str, message: &str) -> impl Future<Output = ()>;
}
/// Agent roster snapshots supplied in their original iteration order.
/// # Implementors
/// Preserve legacy map iteration and parent-scoped managed-agent lookup.
pub trait Listing {
    /// Legacy child names, keys, history sizes and model choices.
    fn legacy_rows(&self) -> Vec<LegacyAgent>;
    /// Managed children visible to the current parent session.
    fn managed_rows(&self) -> Vec<AgentSnapshot>;
    /// Number of current swarm workers.
    fn swarm_count(&self) -> usize;
}
/// Complete command frontend, combining state, registry and listing access.
/// # Implementors
/// Every implementation of the three focused interfaces implements this trait.
pub trait Host: State + Registry + Listing {}
impl<T: State + Registry + Listing> Host for T {}
