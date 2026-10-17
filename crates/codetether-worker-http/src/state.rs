//! Shared worker HTTP state, including task delivery and bus attachment.
use super::*;

/// Worker server state shared across handlers
///
/// # Examples
/// ```
/// let state = codetether_worker_http::WorkerServerState::new();
/// assert!(state.heartbeat_state.is_none());
/// ```
#[derive(Clone)]
pub struct WorkerServerState {
    /// Heartbeat state from the worker (used to determine readiness)
    pub heartbeat_state: Option<Arc<HeartbeatState>>,
    /// Whether the SSE stream is currently connected
    pub connected: Arc<Mutex<bool>>,
    /// Worker ID for identification
    pub worker_id: Arc<Mutex<Option<String>>>,
    /// The actual heartbeat state (internal, for sharing with HTTP server)
    pub(super) internal_heartbeat: Arc<Mutex<Option<Arc<HeartbeatState>>>>,
    /// Channel to notify worker of new tasks (from CloudEvents)
    pub(super) task_notification_tx: Arc<Mutex<Option<mpsc::Sender<String>>>>,
    /// Agent bus for inter-agent messaging (exposed via /v1/bus/*)
    pub(super) bus: Arc<Mutex<Option<Arc<AgentBus>>>>,
}
