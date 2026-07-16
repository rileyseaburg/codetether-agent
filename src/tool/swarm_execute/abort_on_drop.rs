//! Join-handle guard that aborts a direct swarm worker when its owner is dropped.

use tokio::task::{AbortHandle, JoinError, JoinHandle};

pub(super) struct AbortOnDrop<T> {
    handle: Option<JoinHandle<T>>,
    abort: AbortHandle,
}

impl<T> AbortOnDrop<T> {
    pub(super) fn new(handle: JoinHandle<T>) -> Self {
        let abort = handle.abort_handle();
        Self {
            handle: Some(handle),
            abort,
        }
    }

    pub(super) async fn join(mut self) -> Result<T, JoinError> {
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
