//! Worker state construct operations.
use super::*;

impl Default for WorkerServerState {
    fn default() -> Self {
        Self::new()
    }
}
impl WorkerServerState {
    ///
    /// # Arguments
    /// No arguments.
    ///
    /// # Returns
    /// Empty shared state with disconnected flags and no attached channels.
    ///
    /// # Examples
    /// ```
    /// let state = codetether_worker_http::WorkerServerState::new();
    /// assert!(state.heartbeat_state.is_none());
    /// ```
    pub fn new() -> Self {
        Self {
            heartbeat_state: None,
            connected: Arc::new(Mutex::new(false)),
            worker_id: Arc::new(Mutex::new(None)),
            internal_heartbeat: Arc::new(Mutex::new(None)),
            task_notification_tx: Arc::new(Mutex::new(None)),
            bus: Arc::new(Mutex::new(None)),
        }
    }
}
