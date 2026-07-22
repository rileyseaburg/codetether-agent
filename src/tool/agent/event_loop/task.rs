//! Cancellation-safe ownership of one spawned child-agent turn.

use crate::session::Session;
use anyhow::Result;
use tokio::task::{AbortHandle, JoinError, JoinHandle};

pub(in crate::tool::agent) struct ChildTask {
    handle: Option<JoinHandle<Result<Session>>>,
    abort: AbortHandle,
}

impl ChildTask {
    pub(in crate::tool::agent) fn new(handle: JoinHandle<Result<Session>>) -> Self {
        let abort = handle.abort_handle();
        Self {
            handle: Some(handle),
            abort,
        }
    }

    pub(in crate::tool::agent) fn abort(&self) {
        self.abort.abort();
    }

    pub(in crate::tool::agent) async fn join(mut self) -> Result<Result<Session>, JoinError> {
        self.handle.take().expect("child task already joined").await
    }
}

impl Drop for ChildTask {
    fn drop(&mut self) {
        self.abort.abort();
    }
}

#[cfg(test)]
#[path = "task_tests.rs"]
mod tests;
