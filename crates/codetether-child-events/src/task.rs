//! Cancellation-safe ownership of one spawned child-agent turn.

use anyhow::Result;
use tokio::task::{AbortHandle, JoinError, JoinHandle};

/// Abort-on-drop ownership of a child task returning a native session.
/// # Examples
/// ```rust,no_run
/// # async fn example() {
/// let task = tokio::spawn(async { Ok::<_, anyhow::Error>(()) });
/// let child = codetether_child_events::ChildTask::new(task);
/// assert!(child.join().await.unwrap().is_ok());
/// # }
/// ```
pub struct ChildTask<S> {
    handle: Option<JoinHandle<Result<S>>>,
    abort: AbortHandle,
}

impl<S> ChildTask<S> {
    /// Take ownership of a child task and its cancellation handle.
    pub fn new(handle: JoinHandle<Result<S>>) -> Self {
        let abort = handle.abort_handle();
        Self {
            handle: Some(handle),
            abort,
        }
    }

    /// Request cancellation of the child task.
    pub fn abort(&self) {
        self.abort.abort();
    }

    /// Join once, returning the task result or Tokio join error.
    /// # Errors
    /// Returns the Tokio join error, or preserves the child task error inside the result.
    /// # Panics
    /// Panics if the internally owned handle is absent.
    pub async fn join(mut self) -> Result<Result<S>, JoinError> {
        self.handle.take().expect("child task already joined").await
    }
}

impl<S> Drop for ChildTask<S> {
    fn drop(&mut self) {
        self.abort.abort();
    }
}

#[cfg(test)]
#[path = "task_tests.rs"]
mod tests;
