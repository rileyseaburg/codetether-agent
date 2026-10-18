//! Schedule eligible disconnect retries and reset successful turns.
use crate::{MAX_RECONNECT_ATTEMPTS, StreamRecoveryState, is_stream_disconnect};
impl StreamRecoveryState {
    /// Schedule a retry when the error, prompt and remaining budget allow it.
    ///
    /// # Arguments
    /// * `error` — Failure text to classify.
    /// * `inflight_prompt` — Current prompt retained by the host.
    ///
    /// # Returns
    /// Updates the pending prompt and attempt count in place.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_stream_recovery::StreamRecoveryState::default();
    /// state.schedule("broken pipe", &Some("hello".into()));
    /// assert_eq!(state.stream_reconnect_count, 1);
    /// ```
    pub fn schedule(&mut self, error: &str, inflight_prompt: &Option<String>) {
        if !is_stream_disconnect(error) {
            return;
        }
        if self.stream_reconnect_count >= MAX_RECONNECT_ATTEMPTS {
            self.stream_reconnect_count = 0;
            return;
        }
        let prompt = inflight_prompt.clone();
        let Some(prompt) = prompt else { return };
        self.stream_reconnect_count += 1;
        self.pending_stream_reconnect = Some(prompt);
    }
    /// Clear the retry budget and pending prompt after success.
    ///
    /// # Examples
    /// ```
    /// let mut state = codetether_stream_recovery::StreamRecoveryState::default();
    /// state.reset_on_success();
    /// assert!(state.pending_stream_reconnect.is_none());
    /// ```
    pub fn reset_on_success(&mut self) {
        self.stream_reconnect_count = 0;
        self.pending_stream_reconnect = None;
    }
}
