//! Identity, processing and agent registration.
//! Hosts use WorkerState independently of the bridge transport lifecycle.

impl crate::WorkerState {
    /// Attach a worker identity and mark the bridge connected.
    ///
    /// # Arguments
    /// * `worker_id` — Registered worker identifier.
    /// * `worker_name` — Registered display name.
    ///
    /// # Returns
    /// Updates the worker state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// state.set_worker_bridge("id".into(), "tui".into());
    /// assert!(state.a2a_connected);
    /// ```

    pub fn set_worker_bridge(&mut self, worker_id: String, worker_name: String) {
        self.worker_id = Some(worker_id);
        self.worker_name = Some(worker_name);
        self.a2a_connected = true;
    }
    /// Remember the last processing state reported to the bridge.
    ///
    /// # Arguments
    /// * `processing` — Whether the worker is processing a request.
    ///
    /// # Returns
    /// Updates the worker state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// state.sync_worker_bridge_processing(true);
    /// assert_eq!(state.worker_bridge_processing_state, Some(true));
    /// ```

    pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
        self.worker_bridge_processing_state = Some(processing);
    }
    /// Add an advertised agent name.
    ///
    /// # Arguments
    /// * `name` — Agent name to register.
    ///
    /// # Returns
    /// Updates the worker state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_tui_worker::WorkerState::default();
    /// state.register_worker_agent("helper".into());
    /// assert!(state.worker_bridge_registered_agents.contains("helper"));
    /// ```

    pub fn register_worker_agent(&mut self, name: String) {
        self.worker_bridge_registered_agents.insert(name);
    }
}
