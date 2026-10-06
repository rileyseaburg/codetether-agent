//! Convert task-isolated bus updates into the existing output SSE wire format.

use super::filter::matches_task;
use crate::bus::BusEnvelope;
use axum::response::sse::Event;
use futures::Stream;
use std::convert::Infallible;
use tokio::sync::broadcast::{Receiver, error::RecvError};

pub(super) fn events(
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
