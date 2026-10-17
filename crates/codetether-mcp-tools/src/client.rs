//! Host-provided MCP connection and invocation operations.
use anyhow::Result;
use async_trait::async_trait;
use codetether_mcp_core::types::{CallToolResult, McpTool};
use serde_json::Value;
use std::sync::Arc;
/// MCP operations supplied by the existing policy-aware host client.
///
/// # Implementors
/// Delegate to the host's authorized connection and invocation methods. The
/// wrappers do not bypass, replace or duplicate the client's runtime policy.
#[async_trait]
pub trait Client: Send + Sync + Sized {
    /// Connect to a subprocess using the supplied approval receipt.
    /// # Errors
    /// Returns connection, initialization or authorization errors.
    async fn connect_subprocess_with_approval(
        command: &str,
        args: &[&str],
        approval_id: Option<&str>,
    ) -> Result<Arc<Self>>;
    /// Return the tools advertised by the connected server.
    async fn tools(&self) -> Vec<McpTool>;
    /// Invoke a remote tool through the host's existing policy checks.
    /// # Errors
    /// Returns remote invocation or transport errors.
    async fn call_tool(&self, name: &str, args: Value) -> Result<CallToolResult>;
}
