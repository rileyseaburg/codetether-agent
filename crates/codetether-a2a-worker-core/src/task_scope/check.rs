//! Ordered agent, worker and workspace routing checks.
/// Whether a task is within scope for a given worker.
///
/// Returns `Ok(())` if the task should be processed, or `Err(reason)`
/// with a human-readable rejection reason if it should be skipped.
///
/// # Arguments
/// * `task` - Incoming task JSON envelope.
/// * `worker_id` - Receiving worker identity.
/// * `agent_name` - Receiving agent name.
/// * `workspace_ids` - Registered workspace IDs; empty means unrestricted.
///
/// # Returns
/// Returns `Ok(())` when all targeting checks accept the task.
///
/// # Errors
/// Returns the first mismatched agent, worker, or workspace as a reason string.
///
/// # Examples
/// ```rust
/// use codetether_a2a_worker_core::task_scope::check_task_scope;
/// let task = serde_json::json!({"metadata": {"target_agent_name": "builder"}});
/// assert!(check_task_scope(&task, "worker-1", "builder", &[]).is_ok());
/// assert!(check_task_scope(&task, "worker-1", "reviewer", &[]).is_err());
/// ```
pub fn check_task_scope(
    task: &serde_json::Value,
    worker_id: &str,
    agent_name: &str,
    workspace_ids: &[String],
) -> Result<(), String> {
    let metadata = super::fields::task_metadata(task);

    // ── Agent targeting ──────────────────────────────────────────────
    let target_agent = super::fields::target_agent(task, &metadata);
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
