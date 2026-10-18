//! Application identity, approval, and concrete tool-registration policies.
use codetether_mcp_core::{server_catalog::ServerCatalog, types::JsonRpcError};
use serde_json::Value;
use std::{future::Future, sync::Arc};

/// Application services used by the shared MCP server.
/// # Implementors
/// Applications supply their package identity, approval endpoint policy,
/// fallback tool execution, and registry tool wrapping.
pub trait Application: Send + Sync + 'static {
    /// Concrete registry retained when all application tools are exposed.
    type Registry: Send + Sync + 'static;
    /// Application package version advertised to MCP clients.
    fn version() -> &'static str;
    /// Optional approval capability exposed during initialization.
    fn approval_capability() -> Option<Value>;
    /// Handle approval methods with the application's existing policy.
    /// # Errors
    /// Returns the application's JSON-RPC error for rejected requests.
    fn approval(
        method: &str,
        params: Option<Value>,
    ) -> impl Future<Output = Result<Value, JsonRpcError>> + Send;
    /// Register concrete tools while retaining application invocation policy.
    fn register_registry(
        catalog: &ServerCatalog,
        registry: &Arc<Self::Registry>,
    ) -> impl Future<Output = ()> + Send;
    /// Register fallback tools with application command and write policy.
    fn register_fallback(catalog: &ServerCatalog) -> impl Future<Output = ()> + Send;
}
