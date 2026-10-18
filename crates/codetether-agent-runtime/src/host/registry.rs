//! Concrete tool-registry operations and global invocation policy.
use codetether_provider_core::ToolDefinition;
use codetether_tool_core::{Tool, ToolResult};
use std::sync::Arc;
/// Native tool storage and invocation authorization.
/// # Implementors
/// Preserve the existing registry ordering and policy evaluator.
#[async_trait::async_trait]
pub trait Registry: Send + Sync + 'static {
    /// Registry owned by each agent.
    type Tools: Send + Sync;
    /// Fetch a registered tool.
    fn tool(tools: &Self::Tools, name: &str) -> Option<Arc<dyn Tool>>;
    /// Register a tool in the existing registry.
    fn register(tools: &mut Self::Tools, tool: Arc<dyn Tool>);
    /// List tool identifiers in native order.
    fn names(tools: &Self::Tools) -> Vec<&str>;
    /// Read tool schemas in native order.
    fn definitions(tools: &Self::Tools) -> Vec<ToolDefinition>;
    /// Evaluate the existing global invocation policy.
    async fn blocked(name: &str, args: &serde_json::Value) -> Option<ToolResult>;
}
