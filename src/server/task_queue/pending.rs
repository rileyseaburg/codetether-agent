//! Non-mutating snapshots of process-local work eligible for delivery.

use super::{KnativeTask, KnativeTaskQueue};

impl KnativeTaskQueue {
    /// Snapshot pending/queued tasks without acquiring a worker lease.
    ///
    /// This is process-local state, not durable history. Delivery and claiming
    /// must recheck current state because this snapshot may become stale.
    pub(crate) async fn snapshot_pending(&self) -> Vec<KnativeTask> {
        self.tasks
            .lock()
            .await
            .iter()
            .filter(|task| matches!(task.status.as_str(), "pending" | "queued"))
            .cloned()
            .collect()
    }
}
