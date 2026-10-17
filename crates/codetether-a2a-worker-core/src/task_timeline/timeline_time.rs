//! Focused task timeline operations.
use super::{TaskCheckpoint, TaskTimeline};

impl TaskTimeline {
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
