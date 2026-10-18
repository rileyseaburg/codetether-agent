//! Atomic active-to-terminal transitions, independent of HTTP transport.

use super::{KnativeTask, KnativeTaskQueue, TaskCompletion};

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
        self.release_with_completion(task_id, status, TaskCompletion::default())
            .await
    }

    /// Store the winning receipt and terminal status under the same lock.
    ///
    /// # Errors
    /// Returns [`ReleaseError`] without changing either field on rejection.
    pub async fn release_with_completion(
        &self,
        task_id: &str,
        status: &str,
        completion: TaskCompletion,
    ) -> Result<KnativeTask, ReleaseError> {
        self.release_and_notify(task_id, status, completion, |_| {})
            .await
    }
}
