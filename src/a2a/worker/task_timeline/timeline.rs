use std::sync::Arc;
use tokio::{sync::Mutex, time::Instant};

use super::{CheckpointEntry, TaskCheckpoint, TaskProgressState};

/// Tracks the timeline of a task through the processing pipeline.
pub struct TaskTimeline {
    pub(super) task_id: String,
    pub(super) timeout_secs: u64,
    pub(super) start: Instant,
    #[allow(dead_code)]
    pub(super) start_utc: chrono::DateTime<chrono::Utc>,
    pub(super) checkpoints: Vec<CheckpointEntry>,
    pub(super) current: Option<TaskCheckpoint>,
    progress: Arc<Mutex<TaskProgressState>>,
}

impl TaskTimeline {
    pub fn new(task_id: &str, timeout_secs: u64) -> Self {
        let progress = Arc::new(Mutex::new(TaskProgressState {
            task_id: Some(task_id.to_string()),
            remaining_secs: timeout_secs as f64,
            ..Default::default()
        }));
        Self {
            task_id: task_id.to_string(),
            timeout_secs,
            start: Instant::now(),
            start_utc: chrono::Utc::now(),
            checkpoints: Vec::new(),
            current: None,
            progress,
        }
    }
    pub fn checkpoint(&mut self, cp: TaskCheckpoint) { self.checkpoint_with_detail(cp, None); }
    pub fn checkpoint_with_detail(&mut self, cp: TaskCheckpoint, detail: Option<String>) {
        let elapsed_ms = self.start.elapsed().as_millis() as u64;
        let now = chrono::Utc::now();
        let budget = self.budget_pct_used();
        super::diagnostics::log_checkpoint(&self.task_id, self.timeout_secs, cp, elapsed_ms, budget, self.remaining_secs(), detail.as_deref());
        self.current = Some(cp);
        self.checkpoints.push(CheckpointEntry { checkpoint: cp, timestamp: now, elapsed_ms, detail });
    }
}
