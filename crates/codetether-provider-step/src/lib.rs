//! One scoped provider completion with streaming and deadline policy.
//!
//! [complete_step] uses the shared restart engine for structured streaming.
//! Other requests use scoped completion with a configurable wall-clock limit.
//! Stream failures retain their checkpoint data for application persistence.
mod dispatch;
mod timeout;
use anyhow::Result;
use codetether_provider_core::{CompletionRequest, CompletionResponse, Provider};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Complete one provider step using its supported response mode.
///
/// # Arguments
/// * `provider` — Shared provider for this step.
/// * `request` — Prepared completion request.
/// * `session_id` — Provider request scope.
/// * `supports_tools` — Whether this model/request supports tool calls.
/// * `event_tx` — Optional progress stream using the application's snapshot type.
/// # Returns
/// The complete provider response, preserving tool-call and checkpoint metadata.
/// # Errors
/// Propagates provider and stream errors. Non-streaming requests exceeding the
/// configured step deadline return a timeout error.
/// # Examples
/// ```no_run
/// # async fn example(provider: &std::sync::Arc<dyn codetether_provider_core::Provider>, request: codetether_provider_core::CompletionRequest) -> anyhow::Result<()> {
/// let _response = codetether_provider_step::complete_step::<()>(provider, request, "session", true, None).await?;
/// # Ok(()) }
/// ```
pub async fn complete_step<S>(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    session_id: &str,
    supports_tools: bool,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
) -> Result<CompletionResponse> {
    let streaming = supports_tools && provider.supports_structured_streaming();
    let timeout = timeout::step_timeout();
    let fut = dispatch::run(provider, request, session_id, supports_tools, event_tx);
    if streaming {
        return fut.await;
    }
    match tokio::time::timeout(timeout, fut).await {
        Ok(result) => result,
        Err(_) => Err(anyhow::anyhow!(
            "Provider completion exceeded the step timeout of {}s and was aborted. \
             The model stream appears to have stalled.",
            timeout.as_secs()
        )),
    }
}
