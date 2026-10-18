//! Worker identity, advertised agents and incoming task state.
//! Hosts use WorkerState independently of the bridge transport lifecycle.

use crate::IncomingTask;
use std::collections::{HashSet, VecDeque};
/// Host-side state for a TUI worker connection and its task queue.
///
/// # Examples
/// ```
/// let state = codetether_tui_worker::WorkerState::default();
/// assert!(!state.a2a_connected);
/// assert!(state.worker_task_queue.is_empty());
/// ```
#[derive(Debug, Clone, Default)]
pub struct WorkerState {
    /// Identifier assigned to this worker.
    pub worker_id: Option<String>,
    /// Display name registered for this worker.
    pub worker_name: Option<String>,
    /// Whether a worker bridge has been attached.
    pub a2a_connected: bool,
    /// Most recent task labels, retaining at most twenty through push_recent_task.
    pub recent_tasks: Vec<String>,
    /// Agent names advertised or observed by the host.
    pub worker_bridge_registered_agents: HashSet<String>,
    /// Last processing state reported to the bridge.
    pub worker_bridge_processing_state: Option<bool>,
    /// Incoming tasks awaiting execution.
    pub worker_task_queue: VecDeque<IncomingTask>,
    /// Remote task currently being executed by the host.
    pub active_remote_task: Option<IncomingTask>,
}
