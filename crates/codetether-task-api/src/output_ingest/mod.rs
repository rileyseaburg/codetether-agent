//! Worker progress acceptance and ordered output publication.
mod http_error;
mod request;
pub mod service;
pub use request::TaskOutputPayload;
#[cfg(test)]
mod tests;
use axum::{Json, http::StatusCode};
use codetether_bus_core::AgentBus;
use codetether_task_queue::KnativeTaskQueue;
use std::sync::Arc;
/// Accept progress and return its receipt.
/// # Arguments
/// * `tasks` - Process-local queue.
/// * `bus` - Notification bus.
/// * `task_id` - Task receiving progress.
/// * `payload` - Optional output and legacy worker metadata.
/// # Returns
/// Task ID and received status.
/// # Errors
/// Returns 404 for missing tasks and 409 for inactive tasks.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue, b: &std::sync::Arc<codetether_bus_core::AgentBus>, p: &codetether_task_api::output_ingest::TaskOutputPayload) {
/// let _ = codetether_task_api::output_ingest::respond(q, b, "task-1", p).await;
/// # }
/// ```
pub async fn respond(
    tasks: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    task_id: &str,
    payload: &TaskOutputPayload,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    service::ingest(tasks, bus, task_id, payload)
        .await
        .map_err(|error| http_error::response(error, task_id))?;
    Ok(Json(
        serde_json::json!({"task_id":task_id,"status":"received"}),
    ))
}
