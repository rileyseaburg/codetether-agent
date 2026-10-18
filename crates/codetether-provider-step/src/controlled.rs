//! Cancellation-aware completion with parent-goal usage accounting.
use codetether_provider_core::{CompletionRequest, CompletionResponse, Provider};
use codetether_session_control::ControlHandle;
use std::{future::Future, sync::Arc, time::Duration};

/// Durable accounting operation supplied by the owning application.
/// # Implementors
/// Applications record usage in the canonical parent session's goal journal.
pub trait Accounting {
    /// Record the completed response's token sum and elapsed-time increment.
    fn record(
        session_id: &str,
        tokens: usize,
        elapsed: Duration,
    ) -> impl Future<Output = anyhow::Result<()>> + Send;
}

/// Complete a delegated request, honoring cancellation and accounting for usage.
/// # Arguments
/// * `provider` — Provider that receives the original completion request.
/// * `request` — Prepared request, consumed exactly once.
/// * `control` — Optional parent-session control and accounting identity.
/// # Returns
/// The provider response after recording usage for an identified parent session.
/// # Errors
/// Returns cancellation, provider, or accounting errors without replacing them.
/// # Examples
/// ```no_run
/// # async fn example<A: codetether_provider_step::controlled::Accounting>(provider: &std::sync::Arc<dyn codetether_provider_core::Provider>, request: codetether_provider_core::CompletionRequest) -> anyhow::Result<()> {
/// let _response = codetether_provider_step::controlled::complete::<A>(provider, request, None).await?;
/// # Ok(()) }
/// ```
pub async fn complete<A: Accounting>(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    control: Option<&ControlHandle>,
) -> anyhow::Result<CompletionResponse> {
    let response = match control {
        Some(control) => tokio::select! {
            biased;
            _ = control.cancelled() => anyhow::bail!("Session execution cancelled"),
            result = provider.complete(request) => result?,
        },
        None => provider.complete(request).await?,
    };
    if let Some(id) = control.and_then(|control| control.session_id()) {
        let tokens = response
            .usage
            .prompt_tokens
            .saturating_add(response.usage.completion_tokens);
        A::record(id, tokens, Duration::ZERO).await?;
    }
    Ok(response)
}
