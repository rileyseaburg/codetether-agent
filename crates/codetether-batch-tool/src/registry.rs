//! Registry services required for nested batch calls.
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;
/// Tool lookup and policy evaluation supplied by the owning application.
///
/// # Implementors
/// Implement this for the existing registry so batches share its tools and authority.
#[async_trait]
pub trait Registry: Send + Sync {
    /// Look up a registered tool by its identifier.
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>>;
    /// List currently registered identifiers for unknown-tool feedback.
    fn list(&self) -> Vec<&str>;
    /// Return a blocking policy result, or None when a nested call may proceed.
    async fn evaluate_tool_invocation(&self, tool_name: &str, args: &Value) -> Option<ToolResult>;
}
