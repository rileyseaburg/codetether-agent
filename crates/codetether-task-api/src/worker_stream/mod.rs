//! At-least-once worker task notifications with queue-backed lag recovery.

mod delivery;
#[cfg(test)]
mod tests;

use axum::response::sse::Event;
use codetether_bus_core::BusEnvelope;
use codetether_task_queue::{KnativeTask, KnativeTaskQueue};
use std::convert::Infallible;
use tokio::sync::broadcast;

/// At-least-once task notifications; claiming remains a separate queue operation.
/// Broadcast lag replays current queued work, not durable history or worker claims.
/// # Examples
/// ```
/// let queue = codetether_task_queue::KnativeTaskQueue::new();
/// let (_tx, rx) = tokio::sync::broadcast::channel(8);
/// let _stream = codetether_task_api::worker_stream::WorkerStream::new(vec![], rx, queue);
/// ```
pub struct WorkerStream;

impl WorkerStream {
    /// Construct a stream from an initial snapshot and existing subscription.
    /// # Arguments
    /// * `pending` - Initial queued work, emitted in reverse vector order.
    /// * `rx` - Bus subscription established before taking the snapshot.
    /// * `queue` - Authoritative task state rechecked before delivery.
    /// # Returns
    /// Task and lag SSE events until the subscription closes.
    /// # Examples
    /// ```
    /// let queue = codetether_task_queue::KnativeTaskQueue::new();
    /// let (_tx, rx) = tokio::sync::broadcast::channel(8);
    /// let _stream = codetether_task_api::worker_stream::WorkerStream::new(vec![], rx, queue);
    /// ```
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
