//! Completing requests and recording turn latency.
//! Shared timing state and operations are independent of application events.

impl crate::RequestTimingState {
    /// Retain completed request timings and clear the current clock.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// state.complete_request_timing();
    /// ```
    pub fn complete_request_timing(&mut self) {
        self.last_request_first_token_ms = self.current_request_first_token_ms;
        self.last_request_last_token_ms = self.current_request_last_token_ms;
        self.clear_request_timing();
    }
    /// Clear the active request clock and token timings.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// state.clear_request_timing();
    /// ```
    pub fn clear_request_timing(&mut self) {
        self.processing_started_at = None;
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
    }
    /// Record end-to-end turn latency and complete the request.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// state.complete_turn_timing();
    /// ```
    pub fn complete_turn_timing(&mut self) {
        let e2e_ms = self.current_request_elapsed_ms();
        let ttft_ms = self.current_request_first_token_ms;
        if let Some(e2e) = e2e_ms {
            self.chat_latency.record(e2e, ttft_ms);
        }
        self.complete_request_timing();
    }
}
