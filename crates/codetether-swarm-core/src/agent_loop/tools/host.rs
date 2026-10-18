//! Native operations required by the shared sub-agent tool batch.
use super::Completed;
use crate::agent_loop::ToolCall;
use codetether_provider_core::Message;
use std::future::Future;

/// Supply control checks, single-call execution, and transcript storage.
/// # Implementors
/// Native sub-agent state retains its actual tools and control handle.
pub trait Host {
    /// Wait until control permits another tool call.
    fn ready(&mut self) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Execute one call and return its rendered output and attachments.
    fn execute(&mut self, call: ToolCall) -> impl Future<Output = Completed> + Send;
    /// Append one completed tool message through the existing history path.
    fn record(&mut self, message: Message);
}
