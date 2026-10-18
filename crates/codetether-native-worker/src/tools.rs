//! Application registry and runtime-policy adapter for shared tool execution.
use async_trait::async_trait;
use codetether_a2a_worker_core::session_protocol::tool_execution::{self, ToolExecutionHost};
use std::sync::Arc;
struct Host<'a, H: codetether_application_registry::Application>(
    &'a codetether_application_registry::ToolRegistry<H>,
);
#[async_trait]
impl<H: codetether_application_registry::Application> ToolExecutionHost for Host<'_, H> {
    fn tool(&self, name: &str) -> Option<Arc<dyn codetether_tool_core::Tool>> {
        self.0.get(name)
    }
    async fn evaluate(
        &self,
        name: &str,
        input: &serde_json::Value,
    ) -> Option<codetether_tool_core::ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, input).await
    }
}
/// Look up and execute a worker tool under native runtime policy.
/// # Arguments
/// Receives the existing registry, requested tool/input and optional output callback.
/// # Returns
/// The shared execution result, including policy denial or execution failure.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_application_registry::Application>(registry: &codetether_application_registry::ToolRegistry<H>) {
/// let _result = codetether_native_worker::tools::run_tool(registry, "read", serde_json::json!({}), &None).await;
/// # }
/// ```
pub async fn run_tool<H: codetether_application_registry::Application>(
    registry: &codetether_application_registry::ToolRegistry<H>,
    tool_name: &str,
    input: serde_json::Value,
    cb: &Option<Arc<dyn Fn(String) + Send + Sync + 'static>>,
) -> codetether_tool_core::ToolResult {
    tool_execution::run_tool(&Host(registry), tool_name, input, cb).await
}
