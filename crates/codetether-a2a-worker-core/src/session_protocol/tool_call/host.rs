//! Session effects used by the worker tool-call coordinator.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
/// Host-owned argument enrichment, execution, and session result recording.
///
/// # Implementors
/// Application adapters retain workspace/model context and session access.
#[async_trait]
pub trait ToolCallHost: Send {
    /// Prepare approved arguments using the session's model and workspace.
    fn prepare_input(&mut self, input: &serde_json::Value) -> serde_json::Value;
    /// Execute approved, enriched input and return its tool result.
    async fn run_tool(&mut self, tool_name: &str, input: serde_json::Value) -> ToolResult;
    /// Append a tool reply, preserving its provider call ID and attachments.
    fn record_result(&mut self, tool_id: String, result: ToolResult);
}
