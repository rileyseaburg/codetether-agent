//! Canonical application identity, registration, and approval bindings.
use crate::{Native, PackageVersion};
use codetether_application_registry::{Application, ToolRegistry};
use codetether_mcp_core::{server_catalog::ServerCatalog, types::JsonRpcError};
use serde_json::Value;
use std::sync::Arc;
impl<T: Application, V: PackageVersion> codetether_mcp_server::Application for Native<T, V> {
    type Registry = ToolRegistry<T>;
    fn version() -> &'static str {
        V::version()
    }
    fn approval_capability() -> Option<Value> {
        codetether_native_mcp_client::approval::capability()
    }
    async fn approval(method: &str, params: Option<Value>) -> Result<Value, JsonRpcError> {
        codetether_native_mcp_client::approval::handle(method, params).await
    }
    async fn register_registry(catalog: &ServerCatalog, registry: &Arc<Self::Registry>) {
        super::registry::register(catalog, registry).await;
    }
    async fn register_fallback(catalog: &ServerCatalog) {
        super::fallback::register(catalog).await;
    }
}
