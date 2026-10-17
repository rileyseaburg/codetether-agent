//! Serialize task progress using the worker heartbeat wire format.
use super::*;
pub(super) fn payload(progress: TaskProgressState) -> Option<serde_json::Value> {
    progress.task_id.as_ref().map(|_| serde_json::json!({"task_id": progress.task_id, "current_checkpoint": progress.current_checkpoint, "elapsed_secs": format!("{:.1}", progress.elapsed_secs), "remaining_secs": format!("{:.1}", progress.remaining_secs), "budget_pct_used": format!("{:.1}%", progress.budget_pct_used), "checkpoints_reached": progress.checkpoints_reached.len(), "last_detail": progress.last_detail}))
}
