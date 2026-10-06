//! Atomic active-to-terminal transitions, independent of HTTP transport.

use super::{KnativeTask, KnativeTaskQueue};

/// Reasons a worker release cannot change the queue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ReleaseError {
    /// No task has the supplied identifier.
    #[error("task not found")]
    NotFound,
    /// The task is not being processed, or is already terminal.
    #[error("task is not active")]
    NotActive,
    /// A release must use a normalized terminal status.
    #[error("release status must be completed or failed")]
    InvalidStatus,
}

impl KnativeTaskQueue {
    /// Transition an active task to a terminal state exactly once.
    ///
    /// Checking, updating, and snapshotting happen under the same lock.
    /// This does not establish authenticated ownership or durable storage.
    ///
    /// # Errors
    /// Returns [`ReleaseError`] for missing/inactive tasks or invalid statuses.
    pub async fn release(&self, task_id: &str, status: &str) -> Result<KnativeTask, ReleaseError> {
        if !matches!(status, "completed" | "failed") {
            return Err(ReleaseError::InvalidStatus);
        }
        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .iter_mut()
            .find(|task| task.task_id == task_id)
            .ok_or(ReleaseError::NotFound)?;
        if !matches!(task.status.as_str(), "processing" | "working") {
            return Err(ReleaseError::NotActive);
        }
        task.status = status.to_owned();
        Ok(task.clone())
    }
}
