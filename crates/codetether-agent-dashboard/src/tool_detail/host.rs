//! Owner-scoped reads needed for tool-child details.
use codetether_agent_trace::LiveTraceSnapshot;
use codetether_agent_view::AgentSnapshot;
use codetether_provider_core::Message;
/// Read a tool child and its transcript within one parent session.
/// # Implementors
/// Native adapters use their existing registry bridge for each read.
pub trait Host {
    /// Find a child owned by the specified parent.
    fn find(&self, name: &str, parent: &str) -> Option<AgentSnapshot>;
    /// Read persisted transcript messages for that child and parent.
    fn transcript(&self, name: &str, parent: &str) -> Option<Vec<Message>>;
    /// Read any live trace for that child and parent.
    fn trace(&self, name: &str, parent: &str) -> Option<LiveTraceSnapshot>;
}
