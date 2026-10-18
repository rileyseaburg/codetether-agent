//! Convert task-isolated bus updates into the existing output SSE wire format.

use super::filter::matches_task;
use axum::response::sse::Event;
use codetether_bus_core::BusEnvelope;
use futures::Stream;
use std::convert::Infallible;
use tokio::sync::broadcast::{Receiver, error::RecvError};

/// Stream updates and artifacts for exactly one task.
/// # Arguments
/// * `rx` - Subscription receiver.
/// * `task_id` - Required topic and payload task identifier.
/// # Returns
/// Output events until closure, or a gap event followed by closure after lag.
/// # Examples
/// ```
/// let (_, rx) = tokio::sync::broadcast::channel(8);
/// let _stream = codetether_bus_sse::task_events(rx, "task".into());
/// ```
pub fn events(
    mut rx: Receiver<BusEnvelope>,
    task_id: String,
) -> impl Stream<Item = Result<Event, Infallible>> {
    async_stream::stream! {
        loop {
            match rx.recv().await {
                Ok(envelope) => {
                    if !matches_task(&envelope, &task_id) {
                        continue;
                    }
                    if let Ok(data) = serde_json::to_string(&envelope.message) {
                        yield Ok(Event::default().event("output").data(data));
                    }
                }
                Err(RecvError::Lagged(skipped)) => {
                    tracing::warn!(%task_id, skipped, "Task output stream lagged; replay unavailable");
                    let data = serde_json::json!({
                        "task_id": task_id,
                        "error": "output_gap",
                        "skipped_bus_events": skipped,
                        "replay_available": false,
                    }).to_string();
                    yield Ok(Event::default().event("lag").data(data));
                    break;
                }
                Err(RecvError::Closed) => break,
            }
        }
    }
}
