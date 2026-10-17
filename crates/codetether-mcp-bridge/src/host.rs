//! Host authorization hooks for MCP subprocess actions.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Supplies the existing client and application policy decisions.
///
/// # Implementors
/// Delegate both approval hooks to the authoritative runtime policy. Connections
/// still use the supplied client's own authorization and initialization path.
#[async_trait]
pub trait Host: Send + Sync {
    /// Authorized MCP client with resource operations and close support.
    type Client: crate::Resources;
    /// Check whether the invocation has an existing approval receipt.
    fn approved_invocation(tool: &str, args: &Value) -> bool;
    /// Evaluate an invocation, returning a blocking result when denied or pending.
    async fn evaluate_tool_invocation(tool: &str, args: &Value) -> Option<ToolResult>;
}
