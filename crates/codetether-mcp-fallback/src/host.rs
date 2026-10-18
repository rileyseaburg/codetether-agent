//! Native command and policy bindings with shared default search operations.
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Native operations retained by a fallback MCP catalog.
/// # Implementors
/// Applications bind sandboxed commands and their current write-approval check.
/// Search defaults use `codetether-search` and may be overridden by hosts.
/// Handlers outlive catalog registration.
pub trait Host: 'static {
    /// Execute translated BashTool arguments through the existing sandbox.
    /// # Errors
    /// Returns argument or execution errors from the native adapter.
    fn execute_bash(args: Value) -> Result<ToolResult>;
    /// Search file names using the shared native-search implementation.
    /// # Errors
    /// Returns the existing search errors.
    fn search_files(args: Value) -> Result<CallToolResult> {
        super::search::search_files(args)
    }
    /// Search file contents using the shared native-search implementation.
    /// # Errors
    /// Returns the existing search errors.
    fn grep_search(args: Value) -> Result<CallToolResult> {
        super::search::grep_search(args)
    }
    /// Check approval before a command or write; return a blocking result when required.
    fn blocked(name: &str, args: &Value) -> Option<CallToolResult>;
}
