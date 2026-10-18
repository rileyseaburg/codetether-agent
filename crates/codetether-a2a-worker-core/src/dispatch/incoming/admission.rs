//! Task-id and scope admission before reserving execution capacity.
use super::{Handler, reserve::reserve_and_spawn};
use crate::{
    dispatch::{explicitly_targets_agent, targeted_only_enabled},
    initialization::WorkerTaskRuntime,
    task_scope::check_task_scope,
};
/// Reserve a slot and spawn a handler for `task` when it is in scope.
///
/// # Arguments
/// * `task` — Raw task or task envelope received by polling or streaming.
/// * `runtime` — Shared worker identity, scope, capacity, and native services.
/// # Returns
/// Completes after admission/reservation; an admitted task runs in the background.
/// Missing IDs, rejected scope, duplicate tasks, and capacity limits do not spawn.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_a2a_worker_core::dispatch::incoming::Handler>(runtime: &codetether_a2a_worker_core::initialization::WorkerTaskRuntime<H::Client, H::Bus>, task: &serde_json::Value) {
/// codetether_a2a_worker_core::dispatch::incoming::spawn_task_handler::<H>(task, runtime).await;
/// # }
/// ```
pub async fn spawn_task_handler<H: Handler>(
    task: &serde_json::Value,
    runtime: &WorkerTaskRuntime<H::Client, H::Bus>,
) {
    let Some(task_id) = task
        .get("task")
        .and_then(|value| value["id"].as_str())
        .or_else(|| task["id"].as_str())
    else {
        return;
    };
    if targeted_only_enabled() && !explicitly_targets_agent(task, &runtime.agent_name) {
        tracing::debug!(target: "codetether_agent::a2a::worker::task_dispatch",
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
        tracing::debug!(target: "codetether_agent::a2a::worker::task_dispatch", task_id, reason = %reason, "Task skipped — out of scope");
        return;
    }
    reserve_and_spawn::<H>(task, task_id, runtime).await;
}
