//! Session-aware policy callback for inbound MCP tools.
use codetether_tool_core::ToolResult;
use serde_json::Value;
use std::future::Future;
/// Native runtime policy evaluation used before MCP tool execution.
/// # Implementors
/// Hosts retain the current session's context and sandbox checks.
pub trait Evaluator {
    /// Evaluate the original arguments and return a blocking result if denied.
    fn evaluate(name: &str, args: &Value) -> impl Future<Output = Option<ToolResult>>;
}
