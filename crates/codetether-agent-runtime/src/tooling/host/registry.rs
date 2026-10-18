//! Bind existing registry lookups, metadata and policy checks.
use super::*;
#[async_trait::async_trait]
impl<H: Host> codetether_agent_tools::Registry for Agent<H> {
    fn name(&self) -> &str {
        &self.info.name
    }
    fn permission(&self, name: &str) -> Option<&PermissionAction> {
        self.permissions.get(name)
    }
    fn tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        H::tool(&self.tools, name)
    }
    fn names(&self) -> Vec<&str> {
        H::names(&self.tools)
    }
    async fn blocked(name: &str, arguments: &Value) -> Option<ToolResult> {
        H::blocked(name, arguments).await
    }
}
