//! TaskTimeline helper methods.

use std::sync::Arc;
use tokio::sync::Mutex;

use super::{TaskCheckpoint, TaskProgressState, timeline::TaskTimeline};

impl TaskTimeline {
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
        state.checkpoints_reached = self.checkpoints.iter().map(|c| c.checkpoint.as_str().to_string()).collect();
        if let Some(last) = self.checkpoints.last() { state.last_detail = last.detail.clone(); }
    }
    #[allow(dead_code)]
    pub async fn clear_progress(&self) { self.progress.lock().await.clear(); }
    pub fn is_expired(&self) -> bool { self.elapsed_secs() >= self.timeout_secs as f64 }
    pub fn is_near_deadline(&self) -> bool { self.budget_pct_used() >= 80.0 }
    pub fn elapsed_secs(&self) -> f64 { self.start.elapsed().as_secs_f64() }
    pub fn remaining_secs(&self) -> f64 { (self.timeout_secs as f64 - self.elapsed_secs()).max(0.0) }
    pub fn budget_pct_used(&self) -> f64 {
        if self.timeout_secs == 0 { 100.0 } else { (self.elapsed_secs() / self.timeout_secs as f64) * 100.0 }
    }
    #[allow(dead_code)]
    pub fn current(&self) -> Option<TaskCheckpoint> { self.current }
    pub fn reached(&self, cp: TaskCheckpoint) -> bool { self.checkpoints.iter().any(|e| e.checkpoint == cp) }
}
