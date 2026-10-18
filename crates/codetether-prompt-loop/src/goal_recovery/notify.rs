//! Live notification before a goal-level retry delay.
use super::Host;
use codetether_session_events::{SessionEvent, StreamRetryEvent};
pub(super) async fn send(
    host: &mut impl Host,
    error: &anyhow::Error,
    attempt: u8,
    max_restarts: u8,
) {
    let Some(tx) = host.events() else { return };
    let _ = tx
        .send(SessionEvent::StreamRetry(StreamRetryEvent {
            attempt: u32::from(attempt),
            max_restarts: u32::from(max_restarts),
            reason: error.to_string(),
        }))
        .await;
}
