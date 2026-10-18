//! Ephemeral reconnect notification emitted between stream attempts.

use tokio::sync::mpsc;

use codetether_session_events::{SessionEvent, StreamRetryEvent};

use codetether_stream_outcome::StreamStop;

pub(super) async fn retry<S>(
    tx: Option<&mpsc::Sender<SessionEvent<S>>>,
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
