use super::{TaskCheckpoint, TaskTimeline};

pub(super) fn log_checkpoint(
    task_id: &str,
    timeout_secs: u64,
    cp: TaskCheckpoint,
    elapsed_ms: u64,
    budget: f64,
    remaining_secs: f64,
    detail: Option<&str>,
) {
    tracing::info!(task_id = %task_id, checkpoint = cp.as_str(), elapsed_ms, timeout_secs, budget_pct = format!("{:.1}%", budget), detail = detail.unwrap_or(""), "[timeline] checkpoint reached");
    if budget >= 90.0 {
        tracing::warn!(task_id = %task_id, checkpoint = cp.as_str(), budget_pct = format!("{:.1}%", budget), remaining_secs = format!("{:.1}", remaining_secs), "DEADLINE APPROACHING - {:.0}% of time budget consumed", budget);
    } else if budget >= 75.0 {
        tracing::warn!(task_id = %task_id, budget_pct = format!("{:.1}%", budget), "Task consumed {:.0}% of time budget", budget);
    }
}

impl TaskTimeline {
    /// Emit a structured diagnostics summary.
    pub fn emit_diagnostics(&self) {
        let total_ms = self.start.elapsed().as_millis();
        let reached: Vec<&str> = self
            .checkpoints
            .iter()
            .map(|e| e.checkpoint.as_str())
            .collect();
        let skipped: Vec<&str> = TaskCheckpoint::ALL
            .iter()
            .filter(|cp| !self.reached(**cp))
            .map(|cp| cp.as_str())
            .collect();
        let timeline: Vec<String> = self
            .checkpoints
            .iter()
            .map(|e| {
                format!(
                    "{}@{}ms{}",
                    e.checkpoint.as_str(),
                    e.elapsed_ms,
                    e.detail
                        .as_deref()
                        .map(|d| format!("({})", d))
                        .unwrap_or_default()
                )
            })
            .collect();
        let deltas: Vec<String> = (1..self.checkpoints.len())
            .map(|i| {
                format!(
                    "{}->{}: {}ms",
                    self.checkpoints[i - 1].checkpoint.as_str(),
                    self.checkpoints[i].checkpoint.as_str(),
                    self.checkpoints[i].elapsed_ms - self.checkpoints[i - 1].elapsed_ms
                )
            })
            .collect();
        tracing::info!(task_id = %self.task_id, timeout_secs = self.timeout_secs, total_ms, budget_pct = format!("{:.1}%", self.budget_pct_used()), checkpoints_reached = ?reached, checkpoints_skipped = ?skipped, timeline = ?timeline, deltas = ?deltas, "[timeline] task diagnostics summary");
        if self.is_expired() {
            tracing::warn!(task_id = %self.task_id, total_secs = format!("{:.1}", self.elapsed_secs()), timeout_secs = self.timeout_secs, last_checkpoint = self.current.map(|c| c.as_str()).unwrap_or("none"), "TASK EXCEEDED TIME BUDGET - last checkpoint: {}", self.current.map(|c| c.as_str()).unwrap_or("none"));
        }
    }

    /// Serialize diagnostics as JSON for inclusion in task output/release payloads.
    pub fn diagnostics_json(&self) -> serde_json::Value {
        let reached: Vec<serde_json::Value> = self
            .checkpoints
            .iter()
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
        serde_json::json!({ "task_id": self.task_id, "timeout_secs": self.timeout_secs, "total_elapsed_ms": self.start.elapsed().as_millis() as u64, "budget_pct_used": format!("{:.1}%", self.budget_pct_used()), "checkpoints_reached": reached, "checkpoints_skipped": skipped, "deltas": deltas, "expired": self.is_expired() })
    }
}
