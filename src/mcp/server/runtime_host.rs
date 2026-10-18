//! Bind shared MCP dispatch to native invocation and approval policy.
use super::McpServer;
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{JsonRpcError, ServerInfo},
};
use codetether_mcp_transport::Transport;
use serde_json::Value;
use tokio::sync::RwLock;
impl codetether_mcp_server::Host for McpServer {
    fn transport(&self) -> &dyn Transport {
        self.transport.as_ref()
    }
    fn catalog(&self) -> &ServerCatalog {
        &self.catalog
    }
    fn server_info(&self) -> &ServerInfo {
        &self.server_info
    }
    fn initialized(&self) -> &RwLock<bool> {
        &self.initialized
    }
    fn approval_capability(&self) -> Option<Value> {
        crate::mcp::approval::capability()
    }
    async fn setup(&self) {
        self.setup_tools().await;
    }
    async fn call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        self.handle_call_tool(params).await
    }
    async fn approval(&self, method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
        crate::mcp::approval::handle(method, params).await
    }
}
