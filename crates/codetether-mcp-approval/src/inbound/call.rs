//! Synchronous handler bridge and policy-before-execution ordering.
use super::{Evaluator, result::from_tool_result};
use anyhow::Result;
use codetether_mcp_core::types::CallToolResult;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::sync::Arc;
/// Evaluate and execute a registered tool through the synchronous MCP boundary.
/// # Arguments
/// Receives the tool name, executable tool and original argument object.
/// # Returns
/// A converted denial, tool result or execution error envelope.
/// # Errors
/// Execution failures are encoded as MCP tool results rather than propagated.
/// # Panics
/// Panics when the current Tokio runtime cannot support block_in_place.
/// # Examples
/// ```no_run
/// fn call<E: codetether_mcp_approval::inbound::Evaluator>(tool: std::sync::Arc<dyn codetether_tool_core::Tool>) {
///     let _ = codetether_mcp_approval::inbound::call_registered::<E>("read".into(), tool, serde_json::json!({}));
/// }
/// ```
pub fn call_registered<E: Evaluator>(
    name: String,
    tool: Arc<dyn Tool>,
    args: Value,
) -> Result<CallToolResult> {
    let result = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(async {
            if let Some(blocked) = E::evaluate(&name, &args).await {
                return Ok(blocked);
            }
            tool.execute(args).await
        })
    });
    Ok(from_tool_result(result.unwrap_or_else(|error| {
        ToolResult::error(error.to_string())
    })))
}

/// Check policy for a fallback tool before its handler runs.
/// # Arguments
/// Receives the canonical policy name and original arguments.
/// # Returns
/// A converted blocking result, or None when execution may continue.
/// # Panics
/// Panics when the current Tokio runtime cannot support block_in_place.
/// # Examples
/// ```no_run
/// fn check<E: codetether_mcp_approval::inbound::Evaluator>() {
///     let _ = codetether_mcp_approval::inbound::blocked::<E>("read", &serde_json::json!({}));
/// }
/// ```
pub fn blocked<E: Evaluator>(name: &str, args: &Value) -> Option<CallToolResult> {
    tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current()
            .block_on(async { E::evaluate(name, args).await })
            .map(from_tool_result)
    })
}
