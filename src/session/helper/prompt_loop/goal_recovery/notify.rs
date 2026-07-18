//! UI notification for a goal-level provider reconnect.

use super::super::Runner;
use crate::session::{SessionEvent, StreamRetryEvent};

pub(super) async fn send(
    runner: &Runner<'_>,
    error: &anyhow::Error,
    attempt: u8,
    max_restarts: u8,
) {
    let Some(tx) = &runner.events else { return };
    let _ = tx
        .send(SessionEvent::StreamRetry(StreamRetryEvent {
            attempt: u32::from(attempt),
            max_restarts: u32::from(max_restarts),
            reason: error.to_string(),
        }))
        .await;
}
