//! Release coordination; no HTTP concerns. Storage precedes notification.

use super::{ReleaseRequest, outcome};
use crate::{
    bus::{AgentBus, BusMessage},
    server::{KnativeTaskQueue, task_queue::ReleaseError},
};
use std::sync::Arc;

/// Persist the in-process terminal transition before publishing its event.
///
/// # Errors
/// Returns a queue release error without publishing when the transition is rejected.
pub(super) async fn release(
    tasks: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    req: &ReleaseRequest,
) -> Result<&'static str, ReleaseError> {
    let outcome = outcome::from_request(req);
    tasks.release(&req.task_id, outcome.status).await?;
    bus.handle("worker_task_release").send(
        format!("task.{}", req.task_id),
        BusMessage::TaskUpdate {
            task_id: req.task_id.clone(),
            state: outcome.state,
            message: outcome.message,
        },
    );
    tracing::info!(task_id = %req.task_id, status = outcome.status, "Task released by worker");
    Ok(outcome.status)
}
