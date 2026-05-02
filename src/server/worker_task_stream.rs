//! GET /v1/worker/tasks/stream — SSE endpoint for workers.

use super::{AppState, KnativeTask};
use axum::body::Body;
use axum::extract::Request;
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;

#[derive(Debug, Deserialize)]
pub(crate) struct StreamQuery {
    pub agent_name: Option<String>,
    pub worker_id: Option<String>,
}

/// Stream pending tasks as SSE events for connected workers.
///
/// Sends a snapshot of current pending/queued tasks, then keeps the
/// connection alive via the [`AgentBus`](crate::bus::AgentBus) for
/// real-time `TaskUpdate` events.
pub async fn worker_task_stream(
    State(state): State<AppState>,
    Query(_params): Query<StreamQuery>,
    req: Request<Body>,
) -> Sse<impl stream::Stream<Item = Result<Event, Infallible>>> {
    let worker_id = req
        .headers()
        .get("X-Worker-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    tracing::info!(worker_id, "Worker connected to task stream");

    let rx = state.bus.handle("worker_task_stream").into_receiver();
    let pending = snapshot_pending(&state).await;

    let event_stream = super::worker_stream::WorkerStream::new(pending, rx, worker_id);
    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}

async fn snapshot_pending(state: &AppState) -> Vec<KnativeTask> {
    state
        .knative_tasks
        .list()
        .await
        .into_iter()
        .filter(|t| t.status == "pending" || t.status == "queued")
        .collect()
}
