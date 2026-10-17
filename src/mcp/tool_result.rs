//! Compatibility export for shared MCP tool-result conversion.

pub(super) use codetether_mcp_core::tool_result::convert;

#[cfg(test)]
#[path = "tool_result_tests.rs"]
mod tests;
