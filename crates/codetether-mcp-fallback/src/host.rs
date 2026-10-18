//! Native command, search and policy operations for fallback tools.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use serde_json::Value;
/// Native operations retained by a fallback MCP catalog.
/// # Implementors
/// Applications bind sandboxed commands, existing search implementations and
/// their current write-approval check. Handlers outlive catalog registration.
pub trait Host: 'static {
    /// Run a command through the application's existing policy and sandbox.
    /// # Errors
    /// Returns argument or execution errors from the native adapter.
    fn run_command(args: Value) -> Result<CallToolResult>;
    /// Search file names using the application's existing adapter.
    /// # Errors
    /// Returns the existing search errors.
    fn search_files(args: Value) -> Result<CallToolResult>;
    /// Search file contents using the application's existing adapter.
    /// # Errors
    /// Returns the existing search errors.
    fn grep_search(args: Value) -> Result<CallToolResult>;
    /// Check approval before a write; return a blocking result when required.
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult>;
}
