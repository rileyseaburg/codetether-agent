//! Native command, search and policy operations for fallback tools.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Native operations retained by a fallback MCP catalog.
/// # Implementors
/// Applications bind sandboxed commands, existing search implementations and
/// their current write-approval check. Handlers outlive catalog registration.
pub trait Host: 'static {
    /// Execute translated BashTool arguments through the existing sandbox.
    /// # Errors
    /// Returns argument or execution errors from the native adapter.
    fn execute_bash(args: Value) -> Result<ToolResult>;
    /// Search file names using the application's existing adapter.
    /// # Errors
    /// Returns the existing search errors.
    fn search_files(args: Value) -> Result<CallToolResult>;
    /// Search file contents using the application's existing adapter.
    /// # Errors
    /// Returns the existing search errors.
    fn grep_search(args: Value) -> Result<CallToolResult>;
    /// Check approval before a command or write; return a blocking result when required.
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult>;
}
