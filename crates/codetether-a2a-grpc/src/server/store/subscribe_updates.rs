//! subscribe updates task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Subscribe to task update notifications.
    ///
    /// Creates a new receiver for the live task-status broadcast stream. Each
    /// message contains the task ID and the status observed by [`Self::upsert_task`].
    ///
    /// # Returns
    ///
    /// A Tokio broadcast receiver for `(task_id, status)` tuples. Broadcast
    /// receivers are live streams, not durable logs; slow receivers may observe
    /// lag errors according to Tokio broadcast-channel semantics.
    ///
    /// # Side effects
    ///
    /// Registers a new receiver with the broadcast channel. It does not read,
    /// mutate, or replay existing tasks.
    pub fn subscribe_updates(&self) -> broadcast::Receiver<(String, local::TaskStatus)> {
        self.update_tx.subscribe()
    }
}
