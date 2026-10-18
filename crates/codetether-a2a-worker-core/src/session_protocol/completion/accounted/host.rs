use codetether_provider_core::CompletionResponse;
use std::{future::Future, time::Duration};
/// Native goal and provider operations for a worker completion.
/// # Implementors
/// Worker sessions provide their configured model, goal prompt and persistence.
pub trait Host: Send {
    /// Selected provider model identifier.
    fn model(&self) -> &str;
    /// Compose the goal-aware system prompt.
    fn compose(&self) -> String;
    /// Execute completion with the composed prompt and selected temperature.
    /// # Errors
    /// Propagates context preparation or provider failures.
    fn complete(
        &mut self,
        prompt: &str,
        temperature: Option<f32>,
    ) -> impl Future<Output = anyhow::Result<CompletionResponse>> + Send;
    /// Persist consumed tokens and elapsed completion time.
    /// # Errors
    /// Propagates durable goal-accounting failures.
    fn account(
        &mut self,
        tokens: usize,
        elapsed: Duration,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}
