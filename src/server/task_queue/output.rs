//! Process-local progress transitions and ordered synchronous notifications.

use super::{KnativeTask, KnativeTaskQueue};

/// Reasons an output report cannot update the task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum OutputError {
    /// No task has the supplied identifier.
    #[error("task not found")]
    NotFound,
    /// The task no longer accepts progress.
    #[error("task does not accept output")]
    NotActive,
}

impl KnativeTaskQueue {
    /// Mark a nonterminal task working and notify before releasing its lock.
    ///
    /// The synchronous callback must not reenter the queue or block. This
    /// orders process-local bus sends, not durable writes or worker ownership.
    ///
    /// # Errors
    /// Returns [`OutputError`] without mutation or notification on rejection.
    pub async fn record_output<F>(
        &self,
        task_id: &str,
        notify: F,
    ) -> Result<KnativeTask, OutputError>
    where
        F: FnOnce(&KnativeTask),
    {
        let mut tasks = self.tasks.lock().await;
        let task = tasks
            .iter_mut()
            .find(|task| task.task_id == task_id)
            .ok_or(OutputError::NotFound)?;
        if !matches!(
            task.status.as_str(),
            "pending" | "queued" | "processing" | "working"
        ) {
            return Err(OutputError::NotActive);
        }
        task.status = "working".to_owned();
        notify(task);
        Ok(task.clone())
    }
}
