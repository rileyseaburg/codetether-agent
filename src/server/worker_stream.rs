//! SSE stream state machine for [`worker_task_stream`](super::worker_task_stream).

#[path = "worker_stream/delivery.rs"]
mod delivery;
#[cfg(test)]
#[path = "worker_stream/tests.rs"]
mod tests;

use crate::bus::BusEnvelope;
use crate::server::{KnativeTask, KnativeTaskQueue};
use axum::response::sse::Event;
use std::convert::Infallible;
use tokio::sync::broadcast;

/// At-least-once task notifications; claiming remains a separate queue operation.
/// Broadcast lag replays current queued work, not durable history or worker claims.
pub(crate) struct WorkerStream;

impl WorkerStream {
    pub fn new(
        pending: Vec<KnativeTask>,
        rx: broadcast::Receiver<BusEnvelope>,
        queue: KnativeTaskQueue,
    ) -> impl futures::Stream<Item = Result<Event, Infallible>> {
        futures::stream::unfold(
            (pending, rx, queue),
            |(mut pending, mut rx, queue)| async move {
                loop {
                    if let Some(task) = pending.pop() {
                        if let Some(event) = delivery::queued(&queue, &task.task_id).await {
                            return Some((Ok(event), (pending, rx, queue)));
                        }
                        continue;
                    }
                    let event = match rx.recv().await {
                        Ok(envelope) => delivery::live(&queue, envelope).await,
                        Err(broadcast::error::RecvError::Lagged(skipped)) => {
                            // Recover lost notifications; delivery rechecks each task's state.
                            pending = queue.snapshot_pending().await;
                            Some(
                                Event::default()
                                    .event("lag")
                                    .data(format!("{{\"skipped\":{skipped}}}")),
                            )
                        }
                        Err(broadcast::error::RecvError::Closed) => return None,
                    };
                    if let Some(event) = event {
                        return Some((Ok(event), (pending, rx, queue)));
                    }
                }
            },
        )
    }
}
