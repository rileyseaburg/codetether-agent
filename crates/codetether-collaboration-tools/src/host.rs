//! Existing agent execution and canonical-path lookup supplied by the application.
use anyhow::Result;
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Runtime operations required by the collaboration lifecycle tools.
///
/// # Implementors
/// Forward execution and path lookup to the existing application agent engine.
#[async_trait]
pub trait Host: Send + Sync {
    /// Execute an agent payload after trusted context injection.
    /// # Errors
    /// Returns the existing engine's error.
    async fn execute(payload: Value) -> Result<ToolResult>;
    /// Resolve the child path in the requesting session's agent tree.
    /// # Errors
    /// Returns the existing tree lookup error.
    fn canonical(owner: &str, agent_id: &str) -> Result<Option<String>>;
}
