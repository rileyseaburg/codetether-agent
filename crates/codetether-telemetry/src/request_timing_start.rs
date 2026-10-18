//! Starting and observing request clocks.
//! Shared timing state and operations are independent of application events.

use std::time::Instant;
impl crate::RequestTimingState {
    /// Start a request and synchronize its activity timestamp.
    /// # Arguments
    /// * `last_activity` — Watchdog or other observer's last-activity slot.
    /// # Returns
    /// Resets request timing and stores the exact same start instant in the slot.
    /// # Examples
    /// ```
    /// let mut timing = codetether_telemetry::RequestTimingState::default();
    /// let mut activity = None;
    /// timing.begin_with_activity(&mut activity);
    /// assert_eq!(activity, timing.processing_started_at);
    /// ```
    pub fn begin_with_activity(&mut self, last_activity: &mut Option<Instant>) {
        let now = self.begin_request_timing();
        *last_activity = Some(now);
    }

    /// Start a request and reset its current token timings.
    ///
    /// # Returns
    /// The captured start time, so hosts can synchronize other clocks.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// let _started = state.begin_request_timing();
    /// ```
    pub fn begin_request_timing(&mut self) -> Instant {
        let now = Instant::now();
        self.processing_started_at = Some(now);
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
        now
    }
    /// Read elapsed time since this request started.
    ///
    /// # Returns
    /// Elapsed milliseconds, or `None` before a request starts.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// assert_eq!(state.current_request_elapsed_ms(), None);
    /// ```
    pub fn current_request_elapsed_ms(&self) -> Option<u64> {
        self.processing_started_at
            .map(|started| started.elapsed().as_millis() as u64)
    }
}
