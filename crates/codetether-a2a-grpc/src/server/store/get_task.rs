//! get task task-store operation.
use super::*;
impl GrpcTaskStore {
    /// Get a task by id.
    ///
    /// Looks up a task in the in-memory store and returns an owned clone so the
    /// caller can inspect or serialize it without holding a [`DashMap`] guard.
    ///
    /// # Parameters
    ///
    /// * `id` - Task identifier to retrieve.
    ///
    /// # Returns
    ///
    /// `Some(local::Task)` when a task with the given ID exists, or `None` when
    /// the store has no matching task.
    ///
    /// # Side effects
    ///
    /// This method does not mutate store state or emit task updates.
    pub fn get_task(&self, id: &str) -> Option<local::Task> {
        self.tasks.get(id).map(|r| r.value().clone())
    }
}
