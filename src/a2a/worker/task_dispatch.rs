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

fn targeted_only_enabled() -> bool {
    std::env::var("CODETETHER_WORKER_TARGETED_ONLY")
        .ok()
        .is_some_and(|value| matches!(value.trim().to_ascii_lowercase().as_str(), "1" | "true" | "yes" | "on"))
}

fn explicitly_targets_agent(task: &serde_json::Value, agent_name: &str) -> bool {
    task.get("task")
        .and_then(|task| task.get("metadata"))
        .or_else(|| task.get("metadata"))
        .and_then(|metadata| metadata.get("target_agent_name"))
        .or_else(|| task.get("target_agent_name"))
        .or_else(|| task.get("task").and_then(|task| task.get("target_agent_name")))
        .and_then(serde_json::Value::as_str)
        .is_some_and(|target| !target.is_empty() && target == agent_name)
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::explicitly_targets_agent;

    #[test]
    fn targeted_only_accepts_matching_metadata_target() {
        let task = json!({"id":"t1", "metadata":{"target_agent_name":"review-worker"}});
        assert!(explicitly_targets_agent(&task, "review-worker"));
    }

    #[test]
    fn targeted_only_accepts_matching_nested_task_target() {
        let task = json!({"task":{"id":"t1", "target_agent_name":"review-worker"}});
        assert!(explicitly_targets_agent(&task, "review-worker"));
    }

    #[test]
    fn targeted_only_rejects_untargeted_or_mismatched_tasks() {
        assert!(!explicitly_targets_agent(&json!({"id":"t1"}), "review-worker"));
        assert!(!explicitly_targets_agent(
            &json!({"id":"t2", "metadata":{"target_agent_name":"other-worker"}}),
            "review-worker",
        ));
    }
}
