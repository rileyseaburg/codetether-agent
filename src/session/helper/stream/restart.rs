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

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;

use super::idle_timeout::drain;

#[path = "restart/notify.rs"]
mod notify;
#[path = "restart/policy.rs"]
mod policy;

pub(crate) use policy::RestartPolicy;

/// Drive a streamed completion with SRP restarts.
///
/// Opens a stream, drains it, and on a restart-eligible stop re-opens a fresh
/// stream (after backoff) until [`RestartPolicy::max_restarts`] is exhausted.
/// Returns the assembled response, or the last error when the budget runs out.
pub(crate) async fn run(
    provider: &Arc<dyn Provider>,
    request: CompletionRequest,
    event_tx: Option<&mpsc::Sender<SessionEvent>>,
    policy: &RestartPolicy,
) -> Result<CompletionResponse> {
    let mut attempt = 0u32;
    loop {
        let stream = provider.complete_stream(request.clone()).await?;
        let outcome = drain(stream, event_tx).await;
        let restart = outcome.stop.restart_eligible()
            && (!outcome.committed() || outcome.stop.restart_over_committed())
            && attempt < policy.max_restarts;
        if !restart {
            return super::accept::accept(outcome, attempt);
        }
        let wait = policy.backoff(attempt);
        attempt += 1;
        notify::retry(event_tx, attempt, policy.max_restarts, &outcome.stop).await;
        tracing::warn!(attempt, stop = ?outcome.stop, wait_secs = wait.as_secs(), "SRP restart");
        tokio::time::sleep(wait).await;
    }
}
