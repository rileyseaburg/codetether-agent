//! Canonical application identity, registration, and approval bindings.
use codetether_mcp_core::{server_catalog::ServerCatalog, types::JsonRpcError};
use serde_json::Value;
use std::sync::Arc;
/// Application policy bound to the public MCP server alias.
pub struct Native;
impl codetether_mcp_server::Application for Native {
    type Registry = crate::tool::ToolRegistry;
    fn version() -> &'static str {
        env!("CARGO_PKG_VERSION")
    }
    fn approval_capability() -> Option<Value> {
        crate::mcp::approval::capability()
    }
    async fn approval(method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
        crate::mcp::approval::handle(method, params).await
    }
    async fn register_registry(catalog: &ServerCatalog, registry: &Arc<Self::Registry>) {
        super::registry::register(catalog, registry).await;
    }
    async fn register_fallback(catalog: &ServerCatalog) {
        super::fallback::register(catalog).await;
    }
}
