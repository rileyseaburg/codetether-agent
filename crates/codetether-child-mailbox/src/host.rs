//! Application persistence and child-turn execution bindings.
use crate::{Item, Snapshot};
use anyhow::Result;
use async_trait::async_trait;
use codetether_collaboration_state::execution::AgentRunGuard;
use codetether_tool_core::ToolResult;
/// Existing application storage and child-message execution services.
///
/// # Implementors
/// Preserve canonical mailbox paths, atomic persistence and child-turn setup.
#[async_trait]
pub trait Host: Send + Sync + 'static {
    /// Load the child's snapshot or construct its empty queue.
    /// # Errors
    /// Returns the existing persistence read error.
    async fn load(child_id: &str) -> Result<Snapshot>;
    /// Persist a snapshot atomically.
    /// # Errors
    /// Returns the existing persistence write error.
    async fn save(snapshot: &Snapshot) -> Result<()>;
    /// Remove the child's durable snapshot.
    /// # Errors
    /// Returns the existing persistence removal error.
    async fn remove(child_id: &str) -> Result<()>;
    /// Execute a claimed message with the acquired execution guard.
    /// # Errors
    /// Returns the existing child-turn startup error.
    async fn execute(agent_id: String, item: Item, guard: AgentRunGuard) -> Result<ToolResult>;
}
