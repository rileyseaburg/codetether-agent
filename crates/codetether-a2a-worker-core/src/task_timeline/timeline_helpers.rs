//! TaskTimeline helper methods.

use std::sync::Arc;
use tokio::sync::Mutex;

use super::{TaskProgressState, timeline::TaskTimeline};

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
}
