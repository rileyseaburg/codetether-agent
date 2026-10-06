//! Atomic pending-to-processing transition, independent of HTTP transport.

use super::{KnativeTask, KnativeTaskQueue};

/// Reason a pending task cannot be claimed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum ClaimError {
    /// No task has the requested ID.
    #[error("task not found")]
    NotFound,
    /// The task has already been claimed or is terminal.
    #[error("task is not pending")]
    NotPending,
}

impl KnativeTaskQueue {
    /// Claim one pending/queued task and return its processing snapshot.
    ///
    /// Checking the state, updating it, and copying the response happen under
    /// one lock. This is not authenticated ownership or durable storage.
    ///
    /// # Errors
    /// Returns [`ClaimError::NotFound`] for unknown IDs and
    /// [`ClaimError::NotPending`] for tasks outside pending/queued states.
    pub async fn claim(&self, task_id: &str) -> Result<KnativeTask, ClaimError> {
        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .iter_mut()
            .find(|task| task.task_id == task_id)
            .ok_or(ClaimError::NotFound)?;
        if !matches!(task.status.as_str(), "pending" | "queued") {
            return Err(ClaimError::NotPending);
        }
        task.status = "processing".to_string();
        Ok(task.clone())
    }
}
