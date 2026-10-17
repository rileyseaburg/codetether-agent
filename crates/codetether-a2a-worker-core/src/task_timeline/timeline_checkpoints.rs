//! Focused task timeline operations.
use super::{CheckpointEntry, TaskCheckpoint, TaskTimeline};

impl TaskTimeline {
    pub fn checkpoint(&mut self, cp: TaskCheckpoint) {
        self.checkpoint_with_detail(cp, None);
    }

    pub fn checkpoint_with_detail(&mut self, cp: TaskCheckpoint, detail: Option<String>) {
        let elapsed_ms = self.start.elapsed().as_millis() as u64;
        let now = chrono::Utc::now();
        let budget = self.budget_pct_used();
        super::diagnostics::log_checkpoint(
            &self.task_id,
            self.timeout_secs,
            cp,
            elapsed_ms,
            budget,
            self.remaining_secs(),
            detail.as_deref(),
        );
        self.current = Some(cp);
        self.checkpoints.push(CheckpointEntry {
            checkpoint: cp,
            timestamp: now,
            elapsed_ms,
            detail,
        });
    }
}
