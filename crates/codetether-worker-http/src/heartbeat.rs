//! Worker state heartbeat operations.
use super::*;
impl WorkerServerState {
    #[allow(dead_code)]
    ///
    /// # Arguments
    /// state supplies optional initial heartbeat data.
    ///
    /// # Returns
    /// The updated builder value.
    ///
    /// # Examples
    /// ```
    /// let state = codetether_worker_http::WorkerServerState::new().with_heartbeat(None);
    /// assert!(state.heartbeat_state.is_none());
    /// ```
    pub fn with_heartbeat(mut self, state: Option<Arc<HeartbeatState>>) -> Self {
        self.heartbeat_state = state.clone();
        self
    }

    /// Set the heartbeat state (called from worker)
    ///
    /// # Arguments
    /// state supplies live heartbeat data.
    ///
    /// # Returns
    /// Completes after replacing the internal heartbeat.
    ///
    /// # Examples
    /// ```
    /// async fn attach(state: &codetether_worker_http::WorkerServerState) {
    /// let heartbeat = codetether_a2a_worker_core::HeartbeatState::new("w".into(), "agent".into());
    /// state.set_heartbeat_state(std::sync::Arc::new(heartbeat)).await;
    /// }
    /// ```
    pub async fn set_heartbeat_state(&self, state: Arc<HeartbeatState>) {
        *self.internal_heartbeat.lock().await = Some(state);
    }

    /// Get the heartbeat state (for HTTP server)
    #[allow(dead_code)]
    ///
    /// # Arguments
    /// Borrows shared state.
    ///
    /// # Returns
    /// The attached heartbeat, or an unknown-worker default if absent.
    ///
    /// # Examples
    /// ```
    /// async fn read(state: &codetether_worker_http::WorkerServerState) {
    /// let heartbeat = state.heartbeat_state().await;
    /// }
    /// ```
    pub async fn heartbeat_state(&self) -> Arc<HeartbeatState> {
        let guard: Option<Arc<HeartbeatState>> = self.internal_heartbeat.lock().await.clone();
        guard.unwrap_or_else(|| {
            // Create a default heartbeat state if not set
            let state = HeartbeatState::new("unknown".to_string(), "unknown".to_string());
            Arc::new(state)
        })
    }
}
