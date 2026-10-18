//! Bind the existing tool registry and invocation-policy evaluator.
use super::*;
#[async_trait::async_trait]
impl codetether_agent_runtime::Registry for Native {
    type Tools = ToolRegistry;
    fn tool(tools: &ToolRegistry, name: &str) -> Option<Arc<dyn Tool>> {
        tools.get(name)
    }
    fn register(tools: &mut ToolRegistry, tool: Arc<dyn Tool>) {
        tools.register(tool);
    }
    fn names(tools: &ToolRegistry) -> Vec<&str> {
        tools.list()
    }
    fn definitions(tools: &ToolRegistry) -> Vec<crate::provider::ToolDefinition> {
        tools.definitions()
    }
    async fn blocked(name: &str, args: &Value) -> Option<ToolResult> {
        codetether_native_runtime_policy::evaluate_tool_invocation(name, args).await
    }
}
