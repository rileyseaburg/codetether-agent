//! Starting and observing request clocks.
//! Shared timing state and operations are independent of application events.

use std::time::Instant;
impl crate::RequestTimingState {
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
