//! Runtime bindings retained by the application MCP server.
use codetether_mcp_core::{
    server_catalog::ServerCatalog,
    types::{JsonRpcError, ServerInfo},
};
use codetether_mcp_transport::Transport;
use serde_json::Value;
use std::future::Future;
use tokio::sync::RwLock;

/// Server state and application-specific invocation policy.
/// # Implementors
/// Native servers expose their existing transport and catalog and keep tool
/// execution, bus publication and approval handling in their current owners.
pub trait Host: Sync {
    /// Existing transport used for receiving and responding.
    fn transport(&self) -> &dyn Transport;
    /// Existing tool/resource catalog.
    fn catalog(&self) -> &ServerCatalog;
    /// Existing server identity and version.
    fn server_info(&self) -> &ServerInfo;
    /// Initialization notification flag.
    fn initialized(&self) -> &RwLock<bool>;
    /// Read application approval capabilities after parameter validation.
    fn approval_capability(&self) -> Option<Value>;
    /// Register configured tools before receiving the first message.
    fn setup(&self) -> impl Future<Output = ()> + Send;
    /// Invoke a tool with existing policy, locks and bus events.
    /// # Errors
    /// Returns the native JSON-RPC parameter, lookup or execution error.
    fn call_tool(
        &self,
        params: Option<Value>,
    ) -> impl Future<Output = Result<Value, JsonRpcError>> + Send;
    /// Handle application approval methods and unknown-method errors.
    /// # Errors
    /// Returns the existing approval or unknown-method error.
    fn approval(
        &self,
        method: &str,
        params: Option<Value>,
    ) -> impl Future<Output = Result<Value, JsonRpcError>> + Send;
}
