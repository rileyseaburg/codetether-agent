//! Liveness keepalive for the provider inference wait.
//!
//! While the provider is "thinking" before its first chunk (slow models like
//! Bedrock Opus or GLM can take 60-90 s), no [`StreamChunk`] arrives and the
//! session emits no [`SessionEvent`]. The TUI watchdog only sees
//! `SessionEvent`s, so without a keepalive it would cancel a healthy request.
//!
//! [`next_with_keepalive`] polls the stream on a short tick and emits a
//! [`SessionEvent::ProviderKeepalive`] between polls, so the *same* event
//! stream the TUI watchdog already consumes reflects in-flight liveness. The
//! caller's total idle budget ([`IDLE_TIMEOUT`]) is unchanged — keepalives only
//! refresh the UI watchdog, not the displayed activity clock or cold-stall budget.
//!
//! [`IDLE_TIMEOUT`]: super::idle_timeout::IDLE_TIMEOUT

use std::time::Duration;

use codetether_provider_core::StreamChunk;
use codetether_session_events::SessionEvent;
use futures::StreamExt;
use futures::stream::BoxStream;
use tokio::time::Instant;

/// How often to emit a liveness keepalive while awaiting the next chunk.
const KEEPALIVE_TICK: Duration = Duration::from_secs(10);

/// Outcome of waiting for the next chunk with periodic keepalives.
pub(super) enum Next {
    /// A chunk arrived (or the stream ended with `None`).
    Chunk(Option<StreamChunk>),
    /// The total `budget` elapsed with no chunk.
    IdleTimeout,
}

/// Await the next chunk, emitting provider keepalives every [`KEEPALIVE_TICK`].
///
/// Returns once a chunk arrives, the stream ends, or `budget` is exhausted.
pub(super) async fn next_with_keepalive<S>(
    stream: &mut BoxStream<'static, StreamChunk>,
    event_tx: Option<&tokio::sync::mpsc::Sender<SessionEvent<S>>>,
    budget: Duration,
) -> Next {
    // `tokio::time::Instant` shares tokio's clock with `timeout` below, so the
    // budget depletes in lockstep with the timers (and advances under paused
    // virtual time in tests). A `std::time::Instant` here would never expire
    // under `start_paused` and would drift from the timer clock in production.
    let start = Instant::now();
    loop {
        let remaining = budget.saturating_sub(start.elapsed());
        if remaining.is_zero() {
            return Next::IdleTimeout;
        }
        let tick = KEEPALIVE_TICK.min(remaining);
        match tokio::time::timeout(tick, stream.next()).await {
            Ok(chunk) => return Next::Chunk(chunk),
            Err(_) => {
                if let Some(tx) = event_tx {
                    let _ = tx.send(SessionEvent::ProviderKeepalive).await;
                }
            }
        }
    }
}

#[cfg(test)]
#[path = "tests/keepalive.rs"]
mod tests;
