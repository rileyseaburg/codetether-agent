//! Lifecycle management for accepted mux client tasks.

use std::future::Future;

use crate::client_report::report;
use tokio::task::JoinSet;

pub(super) struct ClientTasks {
    tasks: JoinSet<()>,
}

impl ClientTasks {
    pub(super) fn new() -> Self {
        Self {
            tasks: JoinSet::new(),
        }
    }

    pub(super) fn spawn<F>(&mut self, task: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        self.tasks.spawn(task);
    }

    pub(super) async fn reap(&mut self) {
        if self.tasks.is_empty() {
            std::future::pending::<()>().await;
        }
        if let Some(result) = self.tasks.join_next().await {
            super::client_report::report(result);
        }
        while let Some(result) = self.tasks.try_join_next() {
            report(result);
        }
    }

    pub(super) async fn shutdown(mut self) {
        self.tasks.abort_all();
        while self.tasks.join_next().await.is_some() {}
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.tasks.len()
    }
}

#[cfg(test)]
#[path = "tests/client_tasks.rs"]
mod tests;
