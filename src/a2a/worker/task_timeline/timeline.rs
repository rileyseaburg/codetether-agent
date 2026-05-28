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
    pub fn progress_handle(&self) -> Arc<Mutex<TaskProgressState>> {
        Arc::clone(&self.progress)
    }
    pub fn update_timeout_secs(&mut self, timeout_secs: u64) {
        self.timeout_secs = timeout_secs;
    }
    pub async fn sync_progress(&self) {
        let mut state = self.progress.lock().await;
        state.current_checkpoint = self.current.map(|c| c.as_str().to_string());
        state.elapsed_secs = self.elapsed_secs();
        state.remaining_secs = self.remaining_secs();
        state.budget_pct_used = self.budget_pct_used();
        state.checkpoints_reached = self
            .checkpoints
            .iter()
            .map(|c| c.checkpoint.as_str().to_string())
            .collect();
        if let Some(last) = self.checkpoints.last() {
            state.last_detail = last.detail.clone();
        }
    }
    #[allow(dead_code)]
    pub async fn clear_progress(&self) {
        self.progress.lock().await.clear();
    }
    pub fn is_expired(&self) -> bool {
        self.elapsed_secs() >= self.timeout_secs as f64
    }
    pub fn is_near_deadline(&self) -> bool {
        self.budget_pct_used() >= 80.0
    }
    pub fn elapsed_secs(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
    pub fn remaining_secs(&self) -> f64 {
        (self.timeout_secs as f64 - self.elapsed_secs()).max(0.0)
    }
    pub fn budget_pct_used(&self) -> f64 {
        if self.timeout_secs == 0 {
            100.0
        } else {
            (self.elapsed_secs() / self.timeout_secs as f64) * 100.0
        }
    }
    #[allow(dead_code)]
    pub fn current(&self) -> Option<TaskCheckpoint> {
        self.current
    }
    pub fn reached(&self, cp: TaskCheckpoint) -> bool {
        self.checkpoints.iter().any(|e| e.checkpoint == cp)
    }
}
