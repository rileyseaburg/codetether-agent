//! Output lifecycle coordination; synchronous sends share the queue lock.

use super::request::TaskOutputPayload;
use crate::{
    a2a::types::TaskState,
    bus::{AgentBus, BusMessage},
    server::{KnativeTaskQueue, task_queue::OutputError},
};
use std::sync::Arc;

/// Record progress and order its bus send against terminal notifications.
///
/// # Errors
/// Returns a queue rejection without sending progress or mutating the task.
pub(super) async fn ingest(
    tasks: &KnativeTaskQueue,
    bus: &Arc<AgentBus>,
    task_id: &str,
    payload: &TaskOutputPayload,
) -> Result<(), OutputError> {
    tasks
        .record_output(task_id, |_| {
            if let Some(output) = &payload.output {
                bus.handle("task-output").send(
                    format!("task.{task_id}"),
                    BusMessage::TaskUpdate {
                        task_id: task_id.to_owned(),
                        state: TaskState::Working,
                        message: Some(output.clone()),
                    },
                );
            }
        })
        .await?;
    Ok(())
}
