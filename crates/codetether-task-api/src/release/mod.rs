//! Terminal task transitions, response mapping and ordered bus notifications.
mod http_error;
mod outcome;
mod request;
pub mod service;
pub use request::ReleaseRequest;
#[cfg(test)]
mod tests;
use axum::{Json, http::StatusCode};
use codetether_bus_core::AgentBus;
use codetether_task_queue::KnativeTaskQueue;
use std::sync::Arc;
/// Store a terminal receipt and return the worker response.
/// # Arguments
/// * `tasks` - Process-local queue.
/// * `bus` - Notification bus.
/// * `req` - Worker completion payload.
/// # Returns
/// Task ID and normalized terminal status.
/// # Errors
/// Returns 404 for missing tasks and 409 for inactive tasks.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue, b: &std::sync::Arc<codetether_bus_core::AgentBus>, r: &codetether_task_api::release::ReleaseRequest) {
/// let _ = codetether_task_api::release::respond(q, b, r).await;
/// # }
/// ```
pub async fn respond(
    tasks: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    req: &ReleaseRequest,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let status = service::release(tasks, bus, req)
        .await
        .map_err(|reason| http_error::from_release(&req.task_id, reason))?;
    Ok(Json(
        serde_json::json!({"task_id":req.task_id,"status":status}),
    ))
}
