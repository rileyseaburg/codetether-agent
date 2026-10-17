//! Application effects required when consuming a worker model response.
use crate::session_protocol::ToolCall;
use anyhow::Result;
use async_trait::async_trait;
use codetether_provider_core::Message;
/// Host-owned session, telemetry, control, and tool effects.
///
/// # Implementors
/// Implementations retain application state. The response driver invokes
/// methods sequentially and records the assistant message before tools run.
#[async_trait]
pub trait ResponseHost: Send {
    /// Reject work after cancellation or an execution hold.
    /// # Errors
    /// Returns the host's execution-boundary error.
    async fn ensure_running(&mut self) -> Result<()>;
    /// Record prompt and completion token usage for the selected model.
    fn record_usage(&mut self, model: &str, prompt_tokens: u64, completion_tokens: u64);
    /// Append the complete assistant message to the session.
    fn append_message(&mut self, message: Message);
    /// Execute one tool call and record its result using host approval policy.
    async fn execute_tool_call(&mut self, tool_call: ToolCall);
}
