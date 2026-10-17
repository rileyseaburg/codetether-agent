//! Reconnect lifecycle helpers: outcome logging + backoff/breaker driving.

use std::time::Duration;

use anyhow::Result;

use crate::backoff::Backoff;
use crate::breaker::CircuitBreaker;
use crate::lifecycle::{LifecycleState, next_state};

use crate::ingest::StreamDisconnectReason;

/// Log the connect outcome; returns whether the stream had connected.
pub fn log_outcome(outcome: &Result<StreamDisconnectReason>) -> bool {
    match outcome {
        Ok(StreamDisconnectReason::Ended) => {
            tracing::warn!(target: "codetether_agent::a2a::worker::reconnect_lifecycle", "Stream ended, reconnecting...");
            true
        }
        Ok(StreamDisconnectReason::ReadError(error)) => {
            tracing::warn!(target: "codetether_agent::a2a::worker::reconnect_lifecycle", error = %error, "Stream read failed, reconnecting...");
            true
        }
        Err(error) => {
            tracing::error!(target: "codetether_agent::a2a::worker::reconnect_lifecycle", "Stream error: {error}, reconnecting...");
            false
        }
    }
}

/// Drive the lifecycle state machine and sleep for the backoff delay.
pub async fn apply_lifecycle(backoff: &mut Backoff, breaker: &mut CircuitBreaker, connected: bool) {
    if connected {
        breaker.record_success();
        backoff.reset();
    } else {
        breaker.record_failure();
    }
    let state = next_state(LifecycleState::Connecting, connected, breaker.is_open());
    let delay = backoff.next_delay();
    tracing::debug!(target: "codetether_agent::a2a::worker::reconnect_lifecycle",
        ?state,
        failures = breaker.failure_count(),
        delay_ms = delay.as_millis() as u64,
        "Reconnect lifecycle"
    );
    tokio::time::sleep(delay).await;
}

/// Construct the worker's reconnect backoff (500ms base, 60s cap).
pub fn make_backoff() -> Backoff {
    Backoff::new(Duration::from_millis(500), Duration::from_secs(60))
}
