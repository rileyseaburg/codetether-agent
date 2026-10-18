//! Terminal mutation and synchronous notification in one ordering boundary.

use super::{KnativeTask, KnativeTaskQueue, ReleaseError, TaskCompletion};

impl KnativeTaskQueue {
    /// Store a terminal receipt and notify while holding the task queue lock.
    ///
    /// The synchronous callback must not reenter the queue or block. This
    /// preserves bus-send ordering with [`Self::record_output`]; it does not
    /// acknowledge persistence or authenticate the caller.
    ///
    /// # Errors
    /// Returns [`ReleaseError`] without mutation or notification on rejection.
    pub async fn release_and_notify<F>(
        &self,
        task_id: &str,
        status: &str,
        completion: TaskCompletion,
        notify: F,
    ) -> Result<KnativeTask, ReleaseError>
    where
        F: FnOnce(&KnativeTask),
    {
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
        task.completion = completion;
        notify(task);
        Ok(task.clone())
    }
}
