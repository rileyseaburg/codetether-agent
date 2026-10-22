use super::ParsedCall;
use codetether_provider_core::Message;
use std::future::Future;
/// Native transcript insertion and tool execution bindings.
/// # Implementors
/// The runner supplies persistence-aware insertion and guarded tool dispatch.
pub trait Tools {
    /// Review persisted evidence after executing the response's tool calls.
    fn track_progress(&mut self, step: usize);
    /// Append the provider response through the existing session path.
    fn add_message(&mut self, message: Message);
    /// Execute the parsed tool batch.
    /// # Errors
    /// Propagates tool-dispatch failures.
    fn execute(
        &mut self,
        step: usize,
        calls: Vec<ParsedCall>,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
