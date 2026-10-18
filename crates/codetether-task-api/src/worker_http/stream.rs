//! GET /v1/worker/tasks/stream — SSE endpoint for workers.

use super::WorkerState;
use axum::body::Body;
use axum::extract::Request;
use axum::extract::{Query, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream;
use serde::Deserialize;
use std::convert::Infallible;
use std::time::Duration;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
/// Optional identity fields accepted by the worker notification stream.
/// # Examples
/// ```
/// let query = codetether_task_api::worker_http::StreamQuery {
///     agent_name: None, worker_id: Some("worker-1".into()),
/// };
/// assert_eq!(query.worker_id.as_deref(), Some("worker-1"));
/// ```
pub struct StreamQuery {
    /// Optional agent name retained for request compatibility.
    pub agent_name: Option<String>,
    /// Optional worker identifier retained for request compatibility.
    pub worker_id: Option<String>,
}

/// Stream pending tasks as SSE events for connected workers.
///
/// Sends a snapshot of current pending/queued tasks, then keeps the
/// connection alive via the [`AgentBus`](codetether_bus_core::AgentBus) for
/// real-time `TaskUpdate` events.
/// # Arguments
/// Receives request state, optional identity query fields and the HTTP request.
/// # Returns
/// A queue-backed SSE stream with a 15-second keepalive interval.
/// # Examples
/// ```no_run
/// # fn routes<S: codetether_task_api::worker_http::WorkerState>() -> axum::Router<S> {
/// axum::Router::new().route("/stream", axum::routing::get(
///     codetether_task_api::worker_http::worker_task_stream::<S>,
/// ))
/// # }
/// ```
pub async fn worker_task_stream<S: WorkerState>(
    State(state): State<S>,
    Query(_params): Query<StreamQuery>,
    req: Request<Body>,
) -> Sse<impl stream::Stream<Item = Result<Event, Infallible>>> {
    let worker_id = req
        .headers()
        .get("X-Worker-ID")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();

    tracing::info!(target: "codetether_agent::server::worker_modules::worker_task_stream", worker_id, "Worker connected to task stream");

    let rx = state.bus().handle("worker_task_stream").into_receiver();
    let pending = state.queue().snapshot_pending().await;

    let event_stream = crate::worker_stream::WorkerStream::new(pending, rx, state.into_queue());
    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(15)))
}
