//! What counts as "the peer is making progress" while we poll its task.

use std::time::{Duration, Instant};

use crate::a2a::types::Task;

/// Snapshot of everything that changes when the peer does work.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Fingerprint {
    status_timestamp: Option<String>,
    activity_len: usize,
    history_len: usize,
}

impl Fingerprint {
    fn of(task: &Task) -> Self {
        Self {
            status_timestamp: task.status.timestamp.clone(),
            activity_len: task
                .metadata
                .get("activity")
                .and_then(serde_json::Value::as_array)
                .map_or(0, Vec::len),
            history_len: task.history.len(),
        }
    }
}

/// Idle deadline that restarts whenever the task's fingerprint changes.
pub(super) struct IdleWatch {
    limit: Duration,
    last_progress: Instant,
    seen: Fingerprint,
}

impl IdleWatch {
    pub(super) fn start(task: &Task, limit: Duration) -> Self {
        Self {
            limit,
            last_progress: Instant::now(),
            seen: Fingerprint::of(task),
        }
    }

    /// Record the latest task; returns `true` when the idle budget is spent.
    pub(super) fn observe(&mut self, task: &Task) -> bool {
        let now = Fingerprint::of(task);
        if now != self.seen {
            self.seen = now;
            self.last_progress = Instant::now();
        }
        self.last_progress.elapsed() >= self.limit
    }
}
