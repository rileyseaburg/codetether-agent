//! Build the existing extended task heartbeat payload.
use anyhow::{Context, Result};
/// Prepare the task ID and payload before signing a heartbeat request.
///
/// # Errors
/// Returns an error when progress lacks a non-empty task ID.
pub fn prepare_extended_heartbeat(
    worker_id: &str,
    progress: serde_json::Value,
    lease_seconds: u64,
) -> Result<(String, serde_json::Value)> {
    let task_id = progress
        .get("task_id")
        .and_then(|value| value.as_str())
        .filter(|value| !value.is_empty())
        .context("active task progress missing task_id")?
        .to_string();
    let checkpoint_seq = progress
        .get("checkpoints_reached")
        .and_then(|value| value.as_u64())
        .unwrap_or(1)
        .max(1);
    let status_message = progress
        .get("current_checkpoint")
        .and_then(|value| value.as_str())
        .map(ToString::to_string);
    let payload = serde_json::json!({ "task_id": &task_id, "worker_id": worker_id, "status_message": status_message, "checkpoint": progress, "checkpoint_seq": checkpoint_seq, "lease_extension_seconds": lease_seconds });

    Ok((task_id, payload))
}
