//! Application registry and runtime-policy adapter for shared tool execution.
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::tool_execution::{self, ToolExecutionHost};
use std::sync::Arc;
struct Host<'a>(&'a crate::tool::ToolRegistry);
#[async_trait]
impl ToolExecutionHost for Host<'_> {
    fn tool(&self, name: &str) -> Option<Arc<dyn crate::tool::Tool>> {
        self.0.get(name)
    }
    async fn evaluate(
        &self,
        name: &str,
        input: &serde_json::Value,
    ) -> Option<crate::tool::ToolResult> {
        crate::runtime_policy::evaluate_tool_invocation(name, input).await
    }
}
pub(super) async fn run_tool(
    registry: &crate::tool::ToolRegistry,
    tool_name: &str,
    input: serde_json::Value,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> crate::tool::ToolResult {
    tool_execution::run_tool(&Host(registry), tool_name, input, cb).await
}
