//! Application authorization required immediately before repository mutation.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
use serde_json::Value;
/// Supplies the application's existing session-aware undo authorization.
///
/// # Implementors
/// Delegate to the authoritative runtime policy. Return a blocking result to
/// stop the reset, or `None` only when the invocation may proceed.
#[async_trait]
pub trait Policy: Send + Sync + 'static {
    /// Check the original undo arguments before executing the reset.
    async fn blocked(args: &Value) -> Option<ToolResult>;
}
