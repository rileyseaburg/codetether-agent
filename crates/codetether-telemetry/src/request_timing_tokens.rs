//! Token arrival timing.
//! Shared timing state and operations are independent of application events.

impl crate::RequestTimingState {
    /// Record the first and most recent token arrival times.
    ///
    /// # Returns
    /// Updates the timing state in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_telemetry::RequestTimingState::default();
    /// state.note_text_token();
    /// ```
    pub fn note_text_token(&mut self) {
        let Some(elapsed_ms) = self.current_request_elapsed_ms() else {
            return;
        };
        if self.current_request_first_token_ms.is_none() {
            self.current_request_first_token_ms = Some(elapsed_ms);
        }
        self.current_request_last_token_ms = Some(elapsed_ms);
    }
}
