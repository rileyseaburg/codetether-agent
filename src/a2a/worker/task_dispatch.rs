//! Task dispatch helpers shared by polling and SSE streaming.

use super::WorkerTaskRuntime;
use super::check_task_scope;
use super::task_dispatch_reserve::reserve_and_spawn;
use super::task_targeting::{explicitly_targets_agent, targeted_only_enabled};

/// Reserve a slot and spawn a handler for `task` when it is in scope.
pub(super) async fn spawn_task_handler(task: &serde_json::Value, runtime: &WorkerTaskRuntime) {
    let Some(task_id) = task
        .get("task")
        .and_then(|value| value["id"].as_str())
        .or_else(|| task["id"].as_str())
    else {
        return;
    };
    if targeted_only_enabled() && !explicitly_targets_agent(task, &runtime.agent_name) {
        tracing::debug!(
            task_id,
            agent_name = %runtime.agent_name,
            "Task skipped — worker accepts explicitly targeted tasks only"
        );
        return;
    }
    if let Err(reason) = check_task_scope(
        task,
        &runtime.worker_id,
        &runtime.agent_name,
        &runtime.workspace_ids,
    ) {
        tracing::debug!(task_id, reason = %reason, "Task skipped — out of scope");
        return;
    }
    reserve_and_spawn(task, task_id, runtime).await;
}
