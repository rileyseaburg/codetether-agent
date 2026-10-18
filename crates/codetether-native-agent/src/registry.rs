//! Native tool registry access and invocation authorization.
use super::Host;
use codetether_application_registry::{Application, ToolRegistry};
use codetether_provider_core::ToolDefinition;
use codetether_tool_core::{Tool, ToolResult};
use std::sync::Arc;
#[async_trait::async_trait]
impl<S: Send + Sync + 'static, T: Application> codetether_agent_runtime::Registry for Host<S, T> {
    type Tools = ToolRegistry<T>;
    fn tool(tools: &Self::Tools, name: &str) -> Option<Arc<dyn Tool>> {
        tools.get(name)
    }
    fn register(tools: &mut Self::Tools, tool: Arc<dyn Tool>) {
        tools.register(tool);
    }
    fn names(tools: &Self::Tools) -> Vec<&str> {
        tools.list()
    }
    fn definitions(tools: &Self::Tools) -> Vec<ToolDefinition> {
        tools.definitions()
    }
    async fn blocked(name: &str, args: &serde_json::Value) -> Option<ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
