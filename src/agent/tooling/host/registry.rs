//! Bind existing registry lookups, metadata and policy checks.
use super::*;
#[async_trait::async_trait]
impl codetether_agent_tools::Registry for Agent {
    fn name(&self) -> &str {
        &self.info.name
    }
    fn permission(&self, name: &str) -> Option<&PermissionAction> {
        self.permissions.get(name)
    }
    fn tool(&self, name: &str) -> Option<Arc<dyn Tool>> {
        self.tools.get(name)
    }
    fn names(&self) -> Vec<&str> {
        self.tools.list()
    }
    async fn blocked(name: &str, arguments: &Value) -> Option<ToolResult> {
        super::super::policy_gate::blocked(name, arguments).await
    }
}
