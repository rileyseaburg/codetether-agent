//! SSE stream state machine for [`worker_task_stream`](super::worker_task_stream).

use crate::bus::BusEnvelope;
use crate::server::KnativeTask;
use axum::response::sse::Event;
use std::convert::Infallible;
use std::pin::Pin;
use tokio::sync::broadcast;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;

/// Yields pending tasks then switches to bus-driven live events.
pub(crate) struct WorkerStream {
    pending: Vec<KnativeTask>,
    rx: Pin<
        Box<dyn futures::Stream<Item = Result<BusEnvelope, broadcast::error::RecvError>> + Send>,
    >,
    #[allow(dead_code)]
    worker_id: String,
}

impl WorkerStream {
    pub fn new(
        pending: Vec<KnativeTask>,
        rx: broadcast::Receiver<BusEnvelope>,
        worker_id: String,
    ) -> Self {
        Self {
            pending,
            rx: Box::pin(tokio_stream::wrappers::BroadcastStream::new(rx)),
            worker_id,
        }
    }
}

impl futures::Stream for WorkerStream {
    type Item = Result<Event, Infallible>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // Drain buffered pending tasks first.
        if let Some(task) = self.pending.pop() {
            let payload = serde_json::to_string(&task).unwrap_or_default();
            return std::task::Poll::Ready(Some(Ok(Event::default().event("task").data(payload))));
        }

        // Then wait for live bus events.
        match self.rx.as_mut().poll_next(cx) {
            std::task::Poll::Ready(Some(Ok(envelope))) => {
                let payload = serde_json::to_string(&envelope).unwrap_or_default();
                std::task::Poll::Ready(Some(Ok(Event::default().event("task").data(payload))))
            }
            std::task::Poll::Ready(Some(Err(
                tokio::sync::broadcast::error::RecvError::Lagged(n),
            ))) => std::task::Poll::Ready(Some(Ok(Event::default()
                .event("lag")
                .data(format!("skipped {n}"))))),
            std::task::Poll::Ready(Some(Err(
                tokio::sync::broadcast::error::RecvError::Closed,
            ))) => std::task::Poll::Ready(None),
            std::task::Poll::Ready(None) => std::task::Poll::Ready(None),
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }
}
