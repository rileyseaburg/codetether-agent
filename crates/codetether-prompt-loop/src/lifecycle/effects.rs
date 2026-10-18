use crate::StepFlow;
use codetether_provider_core::CompletionResponse;
use codetether_session_events::SessionResult;
use std::future::Future;
/// Native operations performed between shared provider/tool iterations.
/// # Implementors
/// The prompt runner retains session preparation and final persistence.
pub trait Effects: Send {
    /// Prepare the next step and announce it to observers.
    /// # Errors
    /// Propagates boundary, model-preparation and cost-budget failures.
    fn begin(&mut self, step: usize) -> impl Future<Output = anyhow::Result<()>> + Send;
    /// Normalize provider-specific and textual tool calls.
    fn normalize(
        &mut self,
        response: CompletionResponse,
    ) -> impl Future<Output = CompletionResponse> + Send;
    /// Process the normalized response and select the next flow action.
    /// # Errors
    /// Propagates response handling and tool execution failures.
    fn handle(
        &mut self,
        step: usize,
        response: CompletionResponse,
    ) -> impl Future<Output = anyhow::Result<StepFlow>> + Send;
    /// Clear a step limit that did not end the run.
    fn clear_budget(&mut self);
    /// Persist the session and construct its final result.
    /// # Errors
    /// Propagates final persistence failures.
    fn finish(&mut self) -> impl Future<Output = anyhow::Result<SessionResult>> + Send;
}
