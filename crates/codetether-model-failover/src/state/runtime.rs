//! Schedule retries after runtime failure, retaining state if no candidate exists.
use super::RetryState;
impl RetryState {
    /// Select and record a runtime retry, preserving existing state on no candidate.
    /// # Arguments
    /// * `error` - Runtime failure text.
    /// * `model` - Current model identifier.
    /// * `available` - Available model catalog.
    /// * `prompt` - Original prompt to replay.
    /// # Returns
    /// Updates the counter, model history and pending retry when selection succeeds.
    /// # Examples
    /// ```
    /// let mut state = codetether_model_failover::RetryState::default();
    /// state.schedule_runtime("", None, &[], "prompt");
    /// assert!(state.pending.is_none());
    /// ```
    pub fn schedule_runtime(
        &mut self,
        error: &str,
        model: Option<&str>,
        available: &[String],
        prompt: &str,
    ) {
        let provider = model.and_then(|m| m.split('/').next());
        let pending = self.select(error, model, provider, available, prompt);
        if let Some(pending) = pending {
            self.count += 1;
            self.remember_attempt(model, &pending.target_model);
            self.pending = Some(pending);
        }
    }
}
