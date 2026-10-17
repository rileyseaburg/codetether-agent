//! Host-owned tool lookup and runtime policy checks.
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use std::sync::Arc;
/// Application tool lookup and runtime-policy boundary.
///
/// # Implementors
/// Concrete registries resolve tools; runtime policy can return a blocking
/// result before execution. The driver never bypasses these host operations.
#[async_trait]
pub trait ToolExecutionHost: Sync {
    /// Resolve a tool by the exact requested name.
    fn tool(&self, name: &str) -> Option<Arc<dyn Tool>>;
    /// Return a policy denial or hold result, or None when invocation may proceed.
    async fn evaluate(&self, name: &str, input: &serde_json::Value) -> Option<ToolResult>;
}
