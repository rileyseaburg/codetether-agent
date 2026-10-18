//! Slot reservation and handler spawning for in-scope tasks.

use super::Handler;
use crate::{
    dispatch::{TaskReservation, reserve_task_slot},
    initialization::WorkerTaskRuntime,
    task_config::task_str,
};

/// Reserve a concurrency slot for `task_id` and spawn its handler.
pub(super) async fn reserve_and_spawn<H: Handler>(
    task: &serde_json::Value,
    task_id: &str,
    runtime: &WorkerTaskRuntime<H::Client, H::Bus>,
) {
    match reserve_task_slot(&runtime.processing, task_id, runtime.max_concurrent_tasks).await {
        TaskReservation::Reserved => spawn_handler::<H>(task, task_id, runtime),
        TaskReservation::AlreadyProcessing => {}
        TaskReservation::AtCapacity => {
            tracing::debug!(target: "codetether_agent::a2a::worker::task_dispatch_reserve",
                task_id,
                max_concurrent_tasks = runtime.max_concurrent_tasks,
                "Worker is at task capacity; task will stay pending until a slot frees up"
            )
        }
    }
}

fn spawn_handler<H: Handler>(
    task: &serde_json::Value,
    task_id: &str,
    runtime: &WorkerTaskRuntime<H::Client, H::Bus>,
) {
    let task_id = task_str(task, "id").unwrap_or(task_id).to_string();
    let task = task.clone();
    let runtime = runtime.clone();
    tokio::spawn(async move {
        if let Err(error) = H::handle(&runtime, &task).await {
            tracing::error!(target: "codetether_agent::a2a::worker::task_dispatch_reserve", task_id, error = %error, "Task failed");
        }
        runtime.processing.lock().await.remove(&task_id);
    });
}
