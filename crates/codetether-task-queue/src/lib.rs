//! In-process task storage and atomic worker state transitions.
//!
//! KnativeTaskQueue owns shared task records and completion receipts. Claim,
//! output and release mutations run under one lock; HTTP, authentication and
//! durable persistence remain outside this crate.

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
/// # Examples
/// ```
/// let _queue = codetether_task_queue::KnativeTaskQueue::new();
/// ```
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
