//! Bind shared invocation to the native catalog and optional event bus.
use super::McpServer;
use codetether_mcp_core::types::JsonRpcError;
use serde_json::Value;
impl McpServer {
    pub(super) async fn handle_call_tool(
        &self,
        params: Option<Value>,
    ) -> Result<Value, JsonRpcError> {
        codetether_mcp_server::invoke(&self.inner.catalog, self.inner.agent_bus.as_deref(), params)
            .await
    }
}
