//! Task-output SSE transport with server-level authentication middleware.
//! Tenant ownership, worker authorization, and durable replay are separate contracts.

mod filter;
mod service;
#[cfg(test)]
mod tests;

use super::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::Stream;
use std::convert::Infallible;

/// Return 404 for missing tasks, otherwise stream exact-task bus updates.
///
/// # Errors
/// Returns HTTP 404 when the requested task is not in the queue.
pub(super) async fn handler(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, (StatusCode, String)> {
    state
        .knative_tasks
        .get(&task_id)
        .await
        .ok_or_else(|| (StatusCode::NOT_FOUND, format!("Task {task_id} not found")))?;
    let rx = state.bus.handle("task-stream").into_receiver();
    Ok(Sse::new(service::events(rx, task_id)).keep_alive(KeepAlive::default()))
}
