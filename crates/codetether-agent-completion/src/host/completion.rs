//! Native context-aware completion callback.
use crate::*;
/// Execute the prepared values through the existing session-context service.
/// # Implementors
/// Keep the native context policy and provider request options unchanged.
#[async_trait::async_trait]
pub trait Completion: codetether_agent_replay::Host {
    /// Complete one model step against the current session.
    /// # Errors
    /// Propagates native context-builder and provider errors.
    async fn complete(request: Request<'_, Self::Session>) -> anyhow::Result<CompletionResponse>;
}
