//! Drive retry attempts and preserve completed output items.
use crate::{RestartPolicy, attempt, checkpoint, notify};
use codetether_provider_core::{CompletionRequest, CompletionResponse, Provider};
use codetether_session_events::SessionEvent;
use std::sync::Arc;
use tokio::sync::mpsc;

/// Drive a streamed completion with SRP restarts.
///
/// Opens a stream, drains it, and on a restart-eligible stop re-opens a fresh
/// stream (after backoff) until [`RestartPolicy::max_restarts`] is exhausted.
/// Returns the assembled response, or the last error when the budget runs out.
/// # Arguments
/// Receives the provider, initial request, session scope, optional event sender
/// and retry policy. Completed output items may extend subsequent requests.
/// # Returns
/// The accepted response with retained completed items prepended when needed.
/// # Errors
/// Returns terminal provider/drain failures or retry exhaustion. Completed
/// items remain recoverable through [crate::checkpointed_content].
/// # Examples
/// ```no_run
/// # async fn example(provider: &std::sync::Arc<dyn codetether_provider_core::Provider>, request: codetether_provider_core::CompletionRequest) -> anyhow::Result<()> {
/// let response = codetether_stream_restart::run::<()>(provider, request, "session", None, &Default::default()).await?;
/// # Ok(())
/// # }
/// ```
pub async fn run<S>(
    provider: &Arc<dyn Provider>,
    mut request: CompletionRequest,
    session_id: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent<S>>>,
    policy: &RestartPolicy,
) -> anyhow::Result<CompletionResponse> {
    provider.begin_stream_recovery(session_id);
    let mut retries = 0u32;
    let mut checkpoints = checkpoint::State::default();
    loop {
        let outcome = attempt::run(provider, &request, session_id, event_tx).await;
        let eligible = outcome.stop.restart_eligible()
            && (!outcome.committed() || outcome.stop.restart_over_committed());
        if !eligible {
            return checkpoints.accept(outcome, retries);
        }
        if let Some(response) = checkpoints.absorb(&outcome, &mut request) {
            return Ok(response);
        }
        if retries >= policy.max_restarts {
            if provider.try_stream_fallback(&request, session_id) {
                tracing::warn!(target: "codetether_agent::session::helper::stream::restart",session_id, "Falling back from WebSocket to HTTP streaming");
                retries = 0;
                continue;
            }
            let error = codetether_stream_outcome::exhausted(outcome, retries);
            return Err(checkpoints.wrap(error));
        }
        let wait = policy.delay(retries, &outcome.stop);
        retries += 1;
        notify::retry(event_tx, retries, policy.max_restarts, &outcome.stop).await;
        tracing::warn!(target: "codetether_agent::session::helper::stream::restart",retries, stop = ?outcome.stop, wait_ms = wait.as_millis(), "SRP restart");
        tokio::time::sleep(wait).await;
    }
}
