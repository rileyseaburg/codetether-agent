//! Retry history updates and lifecycle resets.
use super::RetryState;
impl RetryState {
    /// Record source and target model names without changing the counter.
    /// # Arguments
    /// * `current` - Source model; missing values are recorded as unknown.
    /// * `target` - Destination model.
    /// # Returns
    /// Appends both identifiers to the existing history.
    /// # Examples
    /// ```
    /// let mut state = codetether_model_failover::RetryState::default();
    /// state.remember_attempt(None, "target");
    /// assert_eq!(state.attempted_models, vec!["unknown", "target"]);
    /// ```
    pub fn remember_attempt(&mut self, current: Option<&str>, target: &str) {
        self.attempted_models
            .push(current.unwrap_or("unknown").to_string());
        self.attempted_models.push(target.to_string());
    }
    /// Clear attempt history while retaining any pending restore request.
    /// # Examples
    /// ```
    /// let mut state = codetether_model_failover::RetryState::default();
    /// state.clear_attempts();
    /// assert_eq!(state.count, 0);
    /// ```
    /// # Examples
    /// ```
    /// let mut state = codetether_model_failover::RetryState::default();
    /// state.clear();
    /// assert_eq!(state.count, 0);
    /// ```
    pub fn clear_attempts(&mut self) {
        self.count = 0;
        self.attempted_models.clear();
    }
    /// Clear attempt history and the pending retry request.
    pub fn clear(&mut self) {
        self.clear_attempts();
        self.pending = None;
    }
}
