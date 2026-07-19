//! Stream Restart Protocol (SRP) engine.
//!
//! Re-opens a *fresh* provider stream when a drain pass stops for a
//! restart-eligible reason ([`StreamStop::restart_eligible`]). LLM streams are
//! not resumable mid-flight, so SRP never token-stitches a "continue": it
//! re-requests from scratch. Restarts fire when nothing usable was committed
//! (cold stall or transient fault) and also when the byte stream ended before
//! a `Done` chunk ([`StreamStop::PrematureEnd`]) — in that case any partial is
//! discarded so a complete answer replaces the truncated one. Bounded by
//! [`RestartPolicy`].

use std::sync::Arc;
use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;

#[path = "restart/attempt.rs"]
mod attempt;
#[path = "restart/checkpoint.rs"]
mod checkpoint;
#[path = "restart/notify.rs"]
mod notify;
#[path = "restart/policy.rs"]
mod policy;

pub(in crate::session::helper) use checkpoint::content_from_error as checkpointed_content;
pub(crate) use policy::RestartPolicy;

/// Drive a streamed completion with SRP restarts.
///
/// Opens a stream, drains it, and on a restart-eligible stop re-opens a fresh
/// stream (after backoff) until [`RestartPolicy::max_restarts`] is exhausted.
/// Returns the assembled response, or the last error when the budget runs out.
pub(crate) async fn run(
    provider: &Arc<dyn Provider>,
    mut request: CompletionRequest,
    session_id: &str,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
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
                tracing::warn!(session_id, "Falling back from WebSocket to HTTP streaming");
                retries = 0;
                continue;
            }
            let error = super::accept::exhausted(outcome, retries);
            return Err(checkpoints.wrap(error));
        }
        let wait = policy.delay(retries, &outcome.stop);
        retries += 1;
        notify::retry(event_tx, retries, policy.max_restarts, &outcome.stop).await;
        tracing::warn!(retries, stop = ?outcome.stop, wait_ms = wait.as_millis(), "SRP restart");
        tokio::time::sleep(wait).await;
    }
}
