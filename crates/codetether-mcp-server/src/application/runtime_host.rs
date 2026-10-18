//! Bind shared MCP dispatch to native invocation and approval policy.
use super::{Application, ApplicationServer};
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{JsonRpcError, ServerInfo},
};
use codetether_mcp_transport::Transport;
use serde_json::Value;
use tokio::sync::RwLock;
impl<H: Application> crate::Host for ApplicationServer<H> {
    fn transport(&self) -> &dyn Transport {
        self.inner.transport.as_ref()
    }
    fn catalog(&self) -> &ServerCatalog {
        &self.inner.catalog
    }
    fn server_info(&self) -> &ServerInfo {
        &self.inner.server_info
    }
    fn initialized(&self) -> &RwLock<bool> {
        &self.inner.initialized
    }
    fn approval_capability(&self) -> Option<Value> {
        H::approval_capability()
    }
    async fn setup(&self) {
        self.setup_tools().await;
    }
    async fn call_tool(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        self.handle_call_tool(params).await
    }
    async fn approval(&self, method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
        H::approval(method, params).await
    }
}
