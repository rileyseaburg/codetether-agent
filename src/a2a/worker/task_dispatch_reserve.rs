//! Slot reservation and handler spawning for in-scope tasks.

use super::{WorkerTaskRuntime, handle_task, reserve_task_slot, task_str};
use crate::a2a::worker::TaskReservation;

/// Reserve a concurrency slot for `task_id` and spawn its handler.
pub(super) async fn reserve_and_spawn(
    task: &serde_json::Value,
    task_id: &str,
    runtime: &WorkerTaskRuntime,
) {
    match reserve_task_slot(&runtime.processing, task_id, runtime.max_concurrent_tasks).await {
        TaskReservation::Reserved => spawn_handler(task, task_id, runtime),
        TaskReservation::AlreadyProcessing => {}
        TaskReservation::AtCapacity => tracing::debug!(
            task_id,
            max_concurrent_tasks = runtime.max_concurrent_tasks,
            "Worker is at task capacity; task will stay pending until a slot frees up"
        ),
    }
}

fn spawn_handler(task: &serde_json::Value, task_id: &str, runtime: &WorkerTaskRuntime) {
    let task_id = task_str(task, "id").unwrap_or(task_id).to_string();
    let task = task.clone();
    let runtime = runtime.clone();
    tokio::spawn(async move {
        if let Err(error) = handle_task(&runtime, &task).await {
            tracing::error!(task_id, error = %error, "Task failed");
        }
        runtime.processing.lock().await.remove(&task_id);
    });
}
