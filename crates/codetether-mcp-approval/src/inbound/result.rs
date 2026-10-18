//! Preserve the shared tool-result to MCP conversion contract.
use codetether_mcp_core::types::CallToolResult;
use codetether_tool_core::ToolResult;
pub(super) fn from_tool_result(result: ToolResult) -> CallToolResult {
    codetether_mcp_core::tool_result::convert(result)
}
