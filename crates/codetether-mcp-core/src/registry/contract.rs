//! Operations supplied by clients stored in the generic MCP registry.
use crate::types::{CallToolResult, McpTool};
use anyhow::Result;
use serde_json::Value;
use std::future::Future;
/// A client that applies its own authorization before remote tool execution.
///
/// # Implementors
/// Application MCP clients implement this interface to retain policy and identity.
pub trait RegistryClient: Send + Sync + 'static {
    /// Return the client's currently cached remote tools.
    fn tools(&self) -> impl Future<Output = Vec<McpTool>> + Send;
    /// Close the client's transport, preserving its error behavior.
    ///
    /// # Errors
    /// The returned future reports any client-specific close failure.
    fn close(&self) -> impl Future<Output = Result<()>> + Send;
    /// Invoke a tool through the client's existing authorization path.
    ///
    /// # Errors
    /// The returned future reports authorization, request or decoding failures.
    fn call_tool(
        &self,
        name: &str,
        arguments: Value,
    ) -> impl Future<Output = Result<CallToolResult>> + Send;
}
