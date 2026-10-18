//! Native execution binding for shared worker task admission.
use super::WorkerTaskRuntime;
mod host;

/// Admit a task through the shared targeting and capacity rules.
pub(super) async fn spawn_task_handler(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
    codetether_a2a_worker_core::dispatch::incoming::spawn_task_handler::<host::Native>(
        task, runtime,
    )
    .await;
}
