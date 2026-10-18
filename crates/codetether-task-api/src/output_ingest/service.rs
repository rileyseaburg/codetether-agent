//! Output lifecycle coordination; synchronous sends share the queue lock.

use super::request::TaskOutputPayload;
use codetether_a2a_core::TaskState;
use codetether_bus_core::AgentBus;
use codetether_bus_core::BusMessage;
use codetether_task_queue::KnativeTaskQueue;
use codetether_task_queue::OutputError;
use std::sync::Arc;

/// Record progress and order its bus send against terminal notifications.
///
/// # Errors
/// Returns a queue rejection without sending progress or mutating the task.
/// # Arguments
/// * `tasks` - Queue holding the receiving task.
/// * `bus` - Bus receiving progress while the queue lock is held.
/// * `task_id` - Task receiving progress.
/// * `payload` - Optional output and legacy metadata.
/// # Returns
/// Unit when progress was accepted.
/// # Examples
/// ```no_run
/// # async fn example(q: &codetether_task_queue::KnativeTaskQueue, b: &std::sync::Arc<codetether_bus_core::AgentBus>, p: &codetether_task_api::output_ingest::TaskOutputPayload) {
/// let _ = codetether_task_api::output_ingest::service::ingest(q, b, "task-1", p).await;
/// # }
/// ```
pub async fn ingest(
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
