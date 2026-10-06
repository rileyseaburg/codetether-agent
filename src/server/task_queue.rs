//! In-process task storage; persistence and worker ownership are separate contracts.

mod claim;
mod completion;
mod operations;
mod output;
mod pending;
mod release;
mod release_notification;
mod task;
#[cfg(test)]
mod tests;

pub use claim::ClaimError;
pub use completion::TaskCompletion;
pub use output::OutputError;
pub use release::ReleaseError;
use std::sync::Arc;
pub use task::KnativeTask;
use tokio::sync::Mutex;

/// Queue for tasks waiting to be processed.
#[derive(Clone)]
pub struct KnativeTaskQueue {
    tasks: Arc<Mutex<Vec<KnativeTask>>>,
}

impl KnativeTaskQueue {
    /// Construct an empty in-memory queue.
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for KnativeTaskQueue {
    fn default() -> Self {
        Self::new()
    }
}
