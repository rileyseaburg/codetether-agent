//! Session effects used by the worker tool-call coordinator.
use async_trait::async_trait;
use codetether_tool_core::ToolResult;
/// Host-owned argument enrichment, execution, and session result recording.
///
/// # Implementors
/// Application adapters retain workspace/model context and session access.
#[async_trait]
pub trait ToolCallHost: Send {
    /// Return an evidence-backed repeated-edit rejection, if the session has one.
    fn repeated_edit(&self, _tool: &str, _input: &serde_json::Value) -> Option<String> {
        None
    }
    /// Prepare approved arguments using the session's model and workspace.
    fn prepare_input(&mut self, input: &serde_json::Value) -> serde_json::Value;
    /// Execute approved, enriched input and return its tool result.
    async fn run_tool(&mut self, tool_name: &str, input: serde_json::Value) -> ToolResult;
    /// Persist pending messages before a session-editing tool reads the store.
    /// # Errors
    /// Returns persistence failures; the tool must not execute afterward.
    async fn persist_context(&mut self) -> anyhow::Result<()>;
    /// Refresh the owner after a successful session edit, before recording its result.
    /// # Errors
    /// Returns reload failures without treating stale state as current.
    async fn refresh_context(&mut self) -> anyhow::Result<()>;
    /// Append a tool reply, preserving its provider call ID and attachments.
    fn record_result(&mut self, tool_id: String, result: ToolResult);
}
