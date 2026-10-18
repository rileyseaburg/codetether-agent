use crate::{execution_policy::AutoApprove, task_timeline};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

/// Shared dependencies and mutable state reused across task handlers.
/// # Examples
/// ```no_run
/// # fn copy<C: Clone, B>(runtime: codetether_a2a_worker_core::initialization::WorkerTaskRuntime<C, B>) {
/// let second = runtime.clone();
/// assert_eq!(second.worker_id, runtime.worker_id);
/// # }
/// ```
pub struct WorkerTaskRuntime<C, B> {
    /// HTTP client shared through the native client's clone operation.
    pub client: C,
    /// Normalized worker server URL.
    pub server: String,
    /// Optional server authentication token.
    pub token: Option<String>,
    /// Worker identity used for task ownership.
    pub worker_id: String,
    /// Advertised agent name.
    pub agent_name: String,
    /// Task IDs with active processing reservations.
    pub processing: Arc<Mutex<HashSet<String>>>,
    /// Normalized maximum number of concurrent tasks.
    pub max_concurrent_tasks: usize,
    /// Parsed tool approval policy.
    pub auto_approve: AutoApprove,
    /// Shared agent bus.
    pub bus: Arc<B>,
    /// Live task progress timeline.
    pub task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
    /// Server workspace IDs registered by this worker.
    pub workspace_ids: Vec<String>,
}
