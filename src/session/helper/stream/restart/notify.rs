//! Ephemeral reconnect notification emitted between stream attempts.

use tokio::sync::mpsc;

use crate::session::{SessionEvent, StreamRetryEvent};

use super::super::outcome::StreamStop;

pub(super) async fn retry(
    tx: Option<&mpsc::Sender<SessionEvent>>,
    attempt: u32,
    max_restarts: u32,
    stop: &StreamStop,
) {
    let Some(tx) = tx else {
        return;
    };
    let _ = tx
        .send(SessionEvent::StreamRetry(StreamRetryEvent {
            attempt,
            max_restarts,
            reason: format!("{stop:?}"),
        }))
        .await;
}
