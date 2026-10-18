//! Join-handle guard that aborts a direct swarm worker when its owner is dropped.

use tokio::task::{AbortHandle, JoinError, JoinHandle};

/// Own a worker join handle and abort the worker when dropped.
/// # Examples
/// ```no_run
/// # async fn example() {
/// let worker = tokio::spawn(async { 42 });
/// let guard = codetether_swarm_tool::abort_on_drop::AbortOnDrop::new(worker);
/// assert_eq!(guard.join().await.unwrap(), 42);
/// # }
/// ```
pub struct AbortOnDrop<T> {
    handle: Option<JoinHandle<T>>,
    abort: AbortHandle,
}

impl<T> AbortOnDrop<T> {
    /// Own an existing worker handle and retain its abort handle.
    pub fn new(handle: JoinHandle<T>) -> Self {
        let abort = handle.abort_handle();
        Self {
            handle: Some(handle),
            abort,
        }
    }

    /// Await the worker, preserving abort-on-drop during cancellation.
    /// # Errors
    /// Returns the worker join error when it panics or is cancelled.
    /// # Panics
    /// Panics if the internal join handle has already been consumed.
    pub async fn join(mut self) -> Result<T, JoinError> {
        self.handle
            .take()
            .expect("join handle already consumed")
            .await
    }
}

impl<T> Drop for AbortOnDrop<T> {
    fn drop(&mut self) {
        self.abort.abort();
    }
}

#[cfg(test)]
#[path = "abort_on_drop_tests.rs"]
mod tests;
