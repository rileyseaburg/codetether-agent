use super::super::Attempt;
use codetether_provider_core::{CompletionRequest, CompletionResponse};
use std::future::Future;
/// Context preparation and provider dispatch for each completion attempt.
/// # Implementors
/// The native runner retains provider handles and session-derived context.
pub trait Requests: Send {
    /// Prepare initial context and zeroed attempt counters.
    /// # Errors
    /// Propagates initial context derivation failures.
    fn initialize(&mut self, step: usize) -> impl Future<Output = anyhow::Result<Attempt>> + Send;
    /// Build and guard the current provider request.
    /// # Errors
    /// Propagates request preparation or budget failures.
    fn build(
        &mut self,
        attempt: &mut Attempt,
    ) -> impl Future<Output = anyhow::Result<CompletionRequest>> + Send;
    /// Dispatch the prepared request through the current provider.
    /// # Errors
    /// Propagates provider or stream failures.
    fn dispatch(
        &mut self,
        request: CompletionRequest,
    ) -> impl Future<Output = anyhow::Result<CompletionResponse>> + Send;
}
