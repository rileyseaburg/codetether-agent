//! Task dispatch helpers shared by polling and SSE streaming.

use super::{WorkerTaskRuntime, check_task_scope, handle_task, reserve_task_slot, task_str};
use crate::a2a::worker::TaskReservation;

pub(super) async fn spawn_task_handler(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
    let Some(task_id) = task
        .get("task")
        .and_then(|value| value["id"].as_str())
        .or_else(|| task["id"].as_str())
    else {
        return;
    };
    if let Err(reason) = check_task_scope(
        task,
        &runtime.worker_id,
        &runtime.agent_name,
        &runtime.workspace_ids,
    ) {
        tracing::debug!(task_id, reason = %reason, "Task skipped — out of scope");
        return;
    }
    match reserve_task_slot(&runtime.processing, task_id, runtime.max_concurrent_tasks).await {
        TaskReservation::Reserved => {
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
        TaskReservation::AlreadyProcessing => {}
        TaskReservation::AtCapacity => tracing::debug!(
            task_id,
            max_concurrent_tasks = runtime.max_concurrent_tasks,
            "Worker is at task capacity; task will stay pending until a slot frees up"
        ),
    }
}
