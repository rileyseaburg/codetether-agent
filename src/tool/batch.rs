//! Shared batch execution using the application's registry and runtime policy.
use super::{Tool, ToolRegistry, ToolResult};
use serde_json::Value;
use std::sync::Arc;
/// Batch tool bound to the application's registered tools.
///
/// # Examples
/// ```rust
/// let _tool = codetether_agent::tool::batch::BatchTool::new();
/// ```
pub type BatchTool = codetether_batch_tool::BatchTool<ToolRegistry>;
#[async_trait::async_trait]
impl codetether_batch_tool::Registry for ToolRegistry {
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        ToolRegistry::get(self, id)
    }
    fn list(&self) -> Vec<&str> {
        ToolRegistry::list(self)
    }
    async fn evaluate_tool_invocation(&self, tool_name: &str, args: &Value) -> Option<ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(tool_name, args).await
    }
}
#[cfg(test)]
#[path = "batch_policy.rs"]
mod batch_policy;
