//! upsert task task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Insert or update a task.
    ///
    /// Stores the provided task by its ID, replacing any existing task with the
    /// same ID. Before writing the task into the map, this method broadcasts the
    /// task's current status so live subscribers can react to status changes.
    ///
    /// # Parameters
    ///
    /// * `task` - Complete task record to insert or replace. Its `id` field is
    ///   used as the storage key and as the task ID in the broadcast update.
    ///
    /// # Side effects
    ///
    /// Sends a best-effort status update on the broadcast channel and mutates
    /// the in-memory task map. If there are no active receivers, or if receivers
    /// have lagged, the broadcast result is ignored; the task is still stored.
    pub fn upsert_task(&self, task: local::Task) {
        let _ = self.update_tx.send((task.id.clone(), task.status.clone()));
        self.tasks.insert(task.id.clone(), task);
    }
}
