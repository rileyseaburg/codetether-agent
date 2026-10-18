//! Tool lifecycle timestamps and completed outcomes.
//! Shared timing state and operations are independent of application events.

use std::time::Instant;
impl crate::ToolTimingState {
    /// Record a tool name and its start time.
    ///
    /// # Arguments
    /// * `name` — Tool being started.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::ToolTimingState::default();
    /// state.start_pending_tool("read".into());
    /// assert_eq!(state.pending_tool_snapshot().unwrap().0, "read");
    /// ```
    pub fn start_pending_tool(&mut self, name: String) {
        self.pending_tool_name = Some(name);
        self.pending_tool_started_at = Some(Instant::now());
    }
    /// Retain a completed tool result and clear the running marker.
    ///
    /// # Arguments
    /// * `name` — Completed tool name.
    /// * `duration_ms` — Measured duration in milliseconds.
    /// * `success` — Whether execution succeeded.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::ToolTimingState::default();
    /// state.note_tool_completed("read".into(), 25, true);
    /// assert_eq!(state.last_tool_latency_ms, Some(25));
    /// ```
    pub fn note_tool_completed(&mut self, name: String, duration_ms: u64, success: bool) {
        self.last_tool_name = Some(name);
        self.last_tool_latency_ms = Some(duration_ms);
        self.last_tool_success = Some(success);
        self.pending_tool_name = None;
        self.pending_tool_started_at = None;
    }
    /// Borrow the running tool name and its start time.
    ///
    /// # Returns
    /// The borrowed name and start time when both are present.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::ToolTimingState::default();
    /// assert!(state.pending_tool_snapshot().is_none());
    /// ```
    pub fn pending_tool_snapshot(&self) -> Option<(&str, Instant)> {
        self.pending_tool_name
            .as_deref()
            .zip(self.pending_tool_started_at)
    }
}
