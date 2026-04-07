//! Retry scheduling logic.

use std::collections::HashSet;

use super::candidates::smart_switch_candidates;
use super::error_detection::{is_retryable_provider_error, smart_switch_model_key};

/// A pending smart switch retry with the prompt and target model.
#[derive(Debug, Clone)]
pub struct PendingSmartSwitchRetry {
    pub prompt: String,
    pub target_model: String,
}

/// Schedules a smart switch retry if the error is retryable and candidates exist.
pub fn maybe_schedule_smart_switch_retry(
    error_msg: &str,
    current_model: Option<&str>,
    current_provider: Option<&str>,
    available_providers: &[String],
    prompt: &str,
    retry_count: u32,
    attempted_models: &[String],
) -> Option<PendingSmartSwitchRetry> {
    if !is_retryable_provider_error(error_msg) {
        return None;
    }
    let max = super::helpers::smart_switch_max_retries() as u32;
    if retry_count >= max {
        return None;
    }
    let attempted: HashSet<String> =
        attempted_models.iter().map(|m| smart_switch_model_key(m)).collect();
    let candidates = smart_switch_candidates(
        current_model, current_provider, available_providers, &attempted,
    );
    candidates.into_iter().next().map(|target_model| PendingSmartSwitchRetry {
        prompt: prompt.to_string(),
        target_model,
    })
}
