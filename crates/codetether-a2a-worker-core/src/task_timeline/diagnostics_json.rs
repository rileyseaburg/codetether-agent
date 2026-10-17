//! JSON serialization of task diagnostics.

use super::{TaskCheckpoint, TaskTimeline};

impl TaskTimeline {
    /// Serialize diagnostics as JSON for inclusion in task output/release payloads.
    pub fn diagnostics_json(&self) -> serde_json::Value {
        let reached: Vec<serde_json::Value> = self.checkpoints.iter()
            .map(|e| serde_json::json!({ "checkpoint": e.checkpoint.as_str(), "elapsed_ms": e.elapsed_ms, "timestamp": e.timestamp.to_rfc3339(), "detail": e.detail }))
            .collect();
        let skipped: Vec<&str> = TaskCheckpoint::ALL
            .iter()
            .filter(|cp| !self.reached(**cp))
            .map(|cp| cp.as_str())
            .collect();
        let deltas: Vec<serde_json::Value> = (1..self.checkpoints.len())
            .map(|i| serde_json::json!({ "from": self.checkpoints[i-1].checkpoint.as_str(), "to": self.checkpoints[i].checkpoint.as_str(), "delta_ms": self.checkpoints[i].elapsed_ms - self.checkpoints[i - 1].elapsed_ms }))
            .collect();
        serde_json::json!({
            "task_id": self.task_id, "timeout_secs": self.timeout_secs,
            "total_elapsed_ms": self.start.elapsed().as_millis() as u64,
            "budget_pct_used": format!("{:.1}%", self.budget_pct_used()),
            "checkpoints_reached": reached, "checkpoints_skipped": skipped,
            "deltas": deltas, "expired": self.is_expired()
        })
    }
}
