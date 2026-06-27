//! Reconnect lifecycle helpers: outcome logging + backoff/breaker driving.

use std::time::Duration;

use anyhow::Result;

use crate::a2a::stream::backoff::Backoff;
use crate::a2a::stream::breaker::CircuitBreaker;
use crate::a2a::stream::lifecycle::{LifecycleState, next_state};

use super::StreamDisconnectReason;

/// Log the connect outcome; returns whether the stream had connected.
pub(super) fn log_outcome(outcome: &Result<StreamDisconnectReason>) -> bool {
    match outcome {
        Ok(StreamDisconnectReason::Ended) => {
            tracing::warn!("Stream ended, reconnecting...");
            true
        }
        Ok(StreamDisconnectReason::ReadError(error)) => {
            tracing::warn!(error = %error, "Stream read failed, reconnecting...");
            true
        }
        Err(error) => {
            tracing::error!("Stream error: {error}, reconnecting...");
            false
        }
    }
}

/// Drive the lifecycle state machine and sleep for the backoff delay.
pub(super) async fn apply_lifecycle(
    backoff: &mut Backoff,
    breaker: &mut CircuitBreaker,
    connected: bool,
) {
    if connected {
        breaker.record_success();
        backoff.reset();
    } else {
        breaker.record_failure();
    }
    let state = next_state(LifecycleState::Connecting, connected, breaker.is_open());
    let delay = backoff.next_delay();
    tracing::debug!(
        ?state,
        failures = breaker.failure_count(),
        delay_ms = delay.as_millis() as u64,
        "Reconnect lifecycle"
    );
    tokio::time::sleep(delay).await;
}

/// Construct the worker's reconnect backoff (500ms base, 60s cap).
pub(super) fn make_backoff() -> Backoff {
    Backoff::new(Duration::from_millis(500), Duration::from_secs(60))
}
