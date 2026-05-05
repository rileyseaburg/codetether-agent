//! Scope gating for incoming A2A tasks and bus events.
//!
//! Agents must only act on work that falls within their declared scope.
//! This module centralises the "is this task/event for me?" check so
//! every intake path (SSE, polling, bus broadcast) applies the same rules.
//!
//! # Scope dimensions
//!
//! | Dimension       | Where it comes from                    | Reject if                         |
//! |-----------------|----------------------------------------|-----------------------------------|
//! | `target_agent`  | task metadata `target_agent_name`      | non-empty AND ≠ worker agent name |
//! | `target_worker` | task metadata `target_worker_id`        | non-empty AND ≠ worker ID         |
//! | `workspace`     | task `workspace_id` / metadata         | not in worker's registered set*   |
//!
//! \* An empty/absent workspace ID is treated as a global broadcast
//!   (virtual task) and is **always** accepted.

/// Whether a task is within scope for a given worker.
///
/// Returns `Ok(())` if the task should be processed, or `Err(reason)`
/// with a human-readable rejection reason if it should be skipped.
pub fn check_task_scope(
    task: &serde_json::Value,
    worker_id: &str,
    agent_name: &str,
    workspace_ids: &[String],
) -> Result<(), String> {
    let metadata = task_metadata(task);

    // ── Agent targeting ──────────────────────────────────────────────
    let target_agent = metadata
        .get("target_agent_name")
        .and_then(|v| v.as_str())
        .or_else(|| task.get("target_agent_name").and_then(|v| v.as_str()))
        .or_else(|| task.get("task").and_then(|t| t.get("target_agent_name")).and_then(|v| v.as_str()));
    if let Some(target) = target_agent {
        if !target.is_empty() && target != agent_name {
            return Err(format!("target_agent_name={target} ≠ agent={agent_name}"));
        }
    }

    // ── Worker targeting ─────────────────────────────────────────────
    let target_worker = metadata
        .get("target_worker_id")
        .and_then(|v| v.as_str())
        .or_else(|| task.get("target_worker_id").and_then(|v| v.as_str()));
    if let Some(target) = target_worker {
        if !target.is_empty() && target != worker_id {
            return Err(format!("target_worker_id={target} ≠ worker={worker_id}"));
        }
    }

    // ── Workspace scoping ────────────────────────────────────────────
    // Empty / "global" workspace IDs are virtual tasks — always accept.
    let ws_id = task
        .get("workspace_id")
        .and_then(|v| v.as_str())
        .or_else(|| metadata.get("workspace_id").and_then(|v| v.as_str()))
        .unwrap_or("");
    if !ws_id.is_empty() && ws_id != "global" && !workspace_ids.is_empty() {
        if !workspace_ids.iter().any(|id| id == ws_id) {
            return Err(format!("workspace_id={ws_id} not in worker scope"));
        }
    }

    Ok(())
}

/// Extract the metadata map from a task JSON envelope.
fn task_metadata(task: &serde_json::Value) -> serde_json::Map<String, serde_json::Value> {
    task.get("task")
        .and_then(|t| t.get("metadata"))
        .or_else(|| task.get("metadata"))
        .and_then(|m| m.as_object())
        .cloned()
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn accepts_unscoped_task() {
        let task = json!({"id": "t1", "prompt": "hello"});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
    }

    #[test]
    fn rejects_wrong_agent() {
        let task = json!({"id": "t1", "metadata": {"target_agent_name": "planner"}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
    }

    #[test]
    fn accepts_matching_agent() {
        let task = json!({"id": "t1", "metadata": {"target_agent_name": "builder"}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
    }

    #[test]
    fn rejects_wrong_worker() {
        let task = json!({"id": "t1", "metadata": {"target_worker_id": "other-worker"}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
    }

    #[test]
    fn accepts_matching_worker() {
        let task = json!({"id": "t1", "metadata": {"target_worker_id": "w1"}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
    }

    #[test]
    fn rejects_unknown_workspace() {
        let task = json!({"id": "t1", "workspace_id": "ws-999"});
        let workspaces = vec!["ws-1".into(), "ws-2".into()];
        assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_err());
    }

    #[test]
    fn accepts_known_workspace() {
        let task = json!({"id": "t1", "workspace_id": "ws-1"});
        let workspaces = vec!["ws-1".into(), "ws-2".into()];
        assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_ok());
    }

    #[test]
    fn accepts_global_workspace() {
        let task = json!({"id": "t1", "workspace_id": "global"});
        let workspaces = vec!["ws-1".into()];
        assert!(check_task_scope(&task, "w1", "builder", &workspaces).is_ok());
    }

    #[test]
    fn accepts_empty_workspace_when_worker_has_none() {
        let task = json!({"id": "t1"});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
    }

    #[test]
    fn nested_task_metadata_agent() {
        let task = json!({"task": {"id": "t1", "metadata": {"target_agent_name": "builder"}}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_ok());
    }

    #[test]
    fn nested_task_metadata_wrong_agent() {
        let task = json!({"task": {"id": "t1", "metadata": {"target_agent_name": "planner"}}});
        assert!(check_task_scope(&task, "w1", "builder", &[]).is_err());
    }
}
