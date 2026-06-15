//! Stream Restart Protocol (SRP) engine.
//!
//! Re-opens a *fresh* provider stream when a drain pass stops for a
//! restart-eligible reason ([`StreamStop::restart_eligible`]). LLM streams are
//! not resumable mid-flight, so SRP only restarts when nothing usable was
//! committed (cold stall or transient fault) — a clean re-request, never a
//! token-stitched "continue". Restarts are bounded by [`RestartPolicy`].

use std::sync::Arc;
use std::time::Duration;

use anyhow::Result;
use tokio::sync::mpsc;

use crate::provider::{CompletionRequest, CompletionResponse, Provider};
use crate::session::SessionEvent;

use super::idle_timeout::drain;

/// Bounded restart budget with exponential backoff.
#[derive(Debug, Clone)]
pub(crate) struct RestartPolicy {
    pub(crate) max_restarts: u32,
    pub(crate) base_backoff: Duration,
    pub(crate) multiplier: u32,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        Self {
            max_restarts: 3,
            base_backoff: Duration::from_secs(2),
            multiplier: 2,
        }
    }
}

impl RestartPolicy {
    /// Backoff before the `attempt`-th restart (0-based): base * mult^attempt.
    pub(crate) fn backoff(&self, attempt: u32) -> Duration {
        self.base_backoff * self.multiplier.saturating_pow(attempt)
    }
}

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
        if !outcome.stop.restart_eligible() || outcome.committed() || attempt >= policy.max_restarts
        {
            return super::accept::accept(outcome, attempt);
        }
        let wait = policy.backoff(attempt);
        attempt += 1;
        tracing::warn!(attempt, stop = ?outcome.stop, wait_secs = wait.as_secs(), "SRP restart");
        tokio::time::sleep(wait).await;
    }
}
