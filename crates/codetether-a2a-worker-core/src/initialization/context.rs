use super::WorkerTaskRuntime;
use crate::{CognitionHeartbeatConfig, HeartbeatState, task_timeline};
use std::{collections::HashSet, sync::Arc};
use tokio::sync::Mutex;

/// Startup state and owned resources retained by worker entrypoints.
/// # Examples
/// ```no_run
/// # fn keep<A, C, B, P>(context: codetether_a2a_worker_core::initialization::WorkerContext<A, C, B, P>) {
/// assert_eq!(context.worker_id, context.task_runtime.worker_id);
/// # }
/// ```
pub struct WorkerContext<A, C, B, P> {
    /// Native parsed CLI arguments.
    pub args: A,
    /// Worker server URL.
    pub server: String,
    /// Advertised worker name.
    pub name: String,
    /// Stable worker identity.
    pub worker_id: String,
    /// Workspaces shared with task handlers.
    pub shared_codebases: Arc<Mutex<Vec<String>>>,
    /// Active processing reservations.
    pub processing: Arc<Mutex<HashSet<String>>>,
    /// Cognition heartbeat settings.
    pub cognition_heartbeat: CognitionHeartbeatConfig,
    /// Mutable worker heartbeat state.
    pub heartbeat_state: HeartbeatState,
    /// Shared agent bus.
    pub bus: Arc<B>,
    /// Live progress shared with task handlers.
    pub task_progress: Arc<Mutex<task_timeline::TaskProgressState>>,
    /// Shared task execution dependencies.
    pub task_runtime: WorkerTaskRuntime<C, B>,
    /// Held to keep A2A peer background tasks alive; dropping aborts them.
    #[allow(dead_code)]
    pub a2a_peer: Option<P>,
}
