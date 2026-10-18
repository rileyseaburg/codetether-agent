//! Policy-before-transport enforcement for outbound MCP tool calls.
use codetether_mcp_core::types::{CallToolResult, ToolContent};
use serde_json::Value;
use std::future::Future;
/// Native policy evaluator and already-connected MCP transport.
/// # Implementors
/// Clients retain session-aware policy checks and remote connection ownership.
pub trait Host: Sync {
    /// Evaluate unmodified arguments and return denial text when blocked.
    fn blocked(&self, name: &str, arguments: &Value)
    -> impl Future<Output = Option<String>> + Send;
    /// Call the remote tool with local approval metadata removed.
    /// # Errors
    /// Returns transport and protocol errors.
    fn call(
        &self,
        name: &str,
        arguments: Value,
    ) -> impl Future<Output = anyhow::Result<CallToolResult>> + Send;
}
/// Evaluate the original arguments before forwarding a remote tool call.
/// # Arguments
/// Receives the policy/transport host, tool name and owned arguments.
/// # Returns
/// A denial envelope or the remote tool result.
/// # Errors
/// Propagates remote transport/protocol errors unchanged.
/// # Examples
/// ```no_run
/// async fn call<H: codetether_mcp_approval::outbound::Host>(host: &H) {
///     let _ = codetether_mcp_approval::outbound::call_checked(host, "read", serde_json::json!({})).await;
/// }
/// ```
pub async fn call_checked(
    host: &impl Host,
    name: &str,
    arguments: Value,
) -> anyhow::Result<CallToolResult> {
    if let Some(text) = host.blocked(name, &arguments).await {
        return Ok(CallToolResult {
            content: vec![ToolContent::Text { text }],
            is_error: true,
        });
    }
    let arguments = strip_approval_id(arguments);
    host.call(name, arguments).await
}
fn strip_approval_id(mut arguments: Value) -> Value {
    if let Some(map) = arguments.as_object_mut() {
        map.remove("approval_id");
    }
    arguments
}
