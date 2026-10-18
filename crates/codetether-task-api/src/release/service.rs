//! Release coordination; no HTTP concerns. Storage precedes notification.

use super::{ReleaseRequest, outcome};
use codetether_bus_core::AgentBus;
use codetether_bus_core::BusMessage;
use codetether_task_queue::KnativeTaskQueue;
use codetether_task_queue::ReleaseError;
use codetether_task_queue::TaskCompletion;
use std::sync::Arc;

/// Persist the in-process terminal transition before publishing its event.
///
/// # Errors
/// Returns a queue release error without publishing when the transition is rejected.
/// # Arguments
/// * `tasks` - Queue containing the active task.
/// * `bus` - Bus receiving the ordered terminal notification.
/// * `req` - Terminal status and completion receipt.
/// # Returns
/// Normalized completed or failed status after the notification.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue, b: &std::sync::Arc<codetether_bus_core::AgentBus>, r: &codetether_task_api::release::ReleaseRequest) {
/// let _ = codetether_task_api::release::service::release(q, b, r).await;
/// # }
/// ```
pub async fn release(
    tasks: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    req: &ReleaseRequest,
) -> Result<&'static str, ReleaseError> {
    let outcome = outcome::from_request(req);
    tasks
        .release_and_notify(
            &req.task_id,
            outcome.status,
            TaskCompletion {
                result: req.result.clone(),
                error: req.error.clone(),
                session_id: req.session_id.clone(),
                diagnostics: req.diagnostics.clone(),
            },
            |_| {
                bus.handle("worker_task_release").send(
                    format!("task.{}", req.task_id),
                    BusMessage::TaskUpdate {
                        task_id: req.task_id.clone(),
                        state: outcome.state,
                        message: outcome.message,
                    },
                );
            },
        )
        .await?;
    tracing::info!(task_id = %req.task_id, status = outcome.status, "Task released by worker");
    Ok(outcome.status)
}
