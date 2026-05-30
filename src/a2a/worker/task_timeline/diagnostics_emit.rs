//! Emit structured diagnostics summary for a task timeline.

use super::{TaskCheckpoint, TaskTimeline};

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
                        .map(|d| format!("({d})"))
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
            tracing::warn!(task_id = %self.task_id, total_secs = format!("{:.1}", self.elapsed_secs()), timeout_secs = self.timeout_secs, last_checkpoint = self.current.map(|c| c.as_str()).unwrap_or("none"), "TASK EXCEEDED TIME BUDGET");
        }
    }
}
