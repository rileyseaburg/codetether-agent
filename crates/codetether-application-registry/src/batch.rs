//! Nested batch lookup and runtime authorization.
use super::{Application, Tool, ToolRegistry};
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::sync::Arc;
#[async_trait::async_trait]
impl<H: Application> codetether_batch_tool::Registry for ToolRegistry<H> {
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>> {
        ToolRegistry::get(self, id)
    }
    fn list(&self) -> Vec<&str> {
        ToolRegistry::list(self)
    }
    async fn evaluate_tool_invocation(&self, tool_name: &str, args: &Value) -> Option<ToolResult> {
        H::evaluate(tool_name, args).await
    }
}
