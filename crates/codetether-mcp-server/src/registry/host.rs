//! Application registry and policy-handler construction.
use codetether_mcp_core::server_catalog::McpToolHandler;
use codetether_tool_core::Tool;
use std::sync::Arc;
/// Registry access used when exposing application tools through MCP.
/// # Implementors
/// Native adapters list/get existing tools and wrap execution with their policy.
pub trait Host {
    /// List tool identifiers in their existing order.
    fn list(&self) -> Vec<&str>;
    /// Return the existing executable tool by identifier.
    fn get(&self, id: &str) -> Option<Arc<dyn Tool>>;
    /// Construct an owned handler preserving the application's tool policy.
    fn handler(name: String, tool: Arc<dyn Tool>) -> McpToolHandler;
}
