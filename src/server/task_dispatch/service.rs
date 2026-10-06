//! Enqueue before publishing so connected workers can read the full payload.

use super::contract::{DispatchTaskRequest, DispatchTaskResponse};
use crate::a2a::types::TaskState;
use crate::bus::{AgentBus, BusMessage};
use crate::server::{KnativeTask, KnativeTaskQueue};
use std::sync::Arc;

pub(super) async fn enqueue(
    queue: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    request: DispatchTaskRequest,
) -> DispatchTaskResponse {
    let task_id = uuid::Uuid::new_v4().to_string();
    queue
        .push(KnativeTask {
            task_id: task_id.clone(),
            title: request.title,
            description: request.description,
            agent_type: request.agent_type.unwrap_or_else(|| "build".into()),
            model: request.model,
            metadata: request.metadata,
            priority: request.priority.unwrap_or(0),
            received_at: chrono::Utc::now(),
            status: "pending".into(),
            completion: Default::default(),
        })
        .await;
    bus.handle("task-dispatch").send(
        format!("task.{task_id}"),
        BusMessage::TaskUpdate {
            task_id: task_id.clone(),
            state: TaskState::Submitted,
            message: Some("Task accepted into local queue".into()),
        },
    );
    DispatchTaskResponse {
        task_id,
        status: "pending",
        dispatched_via_knative: false,
        dispatch_mode: "local_queue",
        durable: false,
    }
}
