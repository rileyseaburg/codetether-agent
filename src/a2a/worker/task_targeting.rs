//! Explicit task-targeting rules for worker dispatch.

/// Whether the worker should only accept explicitly targeted tasks.
pub(super) fn targeted_only_enabled() -> bool {
    std::env::var("CODETETHER_WORKER_TARGETED_ONLY")
        .ok()
        .is_some_and(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
        })
}

/// Whether `task` names `agent_name` as its target agent.
pub(super) fn explicitly_targets_agent(task: &serde_json::Value, agent_name: &str) -> bool {
    task.get("task")
        .and_then(|task| task.get("metadata"))
        .or_else(|| task.get("metadata"))
        .and_then(|metadata| metadata.get("target_agent_name"))
        .or_else(|| task.get("target_agent_name"))
        .or_else(|| {
            task.get("task")
                .and_then(|task| task.get("target_agent_name"))
        })
        .and_then(serde_json::Value::as_str)
        .is_some_and(|target| !target.is_empty() && target == agent_name)
}
