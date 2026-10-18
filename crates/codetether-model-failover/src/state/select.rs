//! Candidate selection using the current retry budget and model history.
use super::RetryState;
use crate::{PendingSmartSwitchRetry, maybe_schedule_smart_switch_retry};
impl RetryState {
    /// Select a retry without changing stored state.
    /// # Arguments
    /// * `error` - Provider failure text.
    /// * `model` - Current model identifier.
    /// * `provider` - Current provider identifier.
    /// * `available` - Available model catalog.
    /// * `prompt` - Original prompt to replay.
    /// # Returns
    /// A retry if classification, budget and available candidates allow it.
    /// # Examples
    /// ```
    /// let state = codetether_model_failover::RetryState::default();
    /// assert!(state.select("", None, None, &[], "prompt").is_none());
    /// ```
    pub fn select(
        &self,
        error: &str,
        model: Option<&str>,
        provider: Option<&str>,
        available: &[String],
        prompt: &str,
    ) -> Option<PendingSmartSwitchRetry> {
        maybe_schedule_smart_switch_retry(
            error,
            model,
            provider,
            available,
            prompt,
            self.count,
            &self.attempted_models,
        )
    }
}
