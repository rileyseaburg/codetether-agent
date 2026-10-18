//! Candidate selection for a provider-error smart retry.

use std::collections::HashSet;

use super::pending::PendingSmartSwitchRetry;
use crate::account_state::is_provider_account_exhausted;
use crate::candidates::smart_switch_candidates;
use crate::error_detection::{is_retryable_provider_error, smart_switch_model_key};
use crate::provider_filter::filter_out_provider;

/// Schedules a smart-switch retry if the error is retryable and candidates exist.
///
/// # Arguments
/// * `error_msg` — error msg input.
/// * `current_model` — current model input.
/// * `current_provider` — current provider input.
/// * `available_providers` — available providers input.
/// * `prompt` — prompt input.
/// * `retry_count` — retry count input.
/// * `attempted_models` — attempted models input.
///
/// # Returns
/// The selected value when the policy permits it, otherwise None.
///
/// # Examples
/// ```
/// assert!(codetether_model_failover::maybe_schedule_smart_switch_retry("not retryable", None, None, &[], "task", 0, &[]).is_none());
/// ```
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
    if retry_count >= super::super::helpers::smart_switch_max_retries() as u32 {
        return None;
    }
    let attempted: HashSet<String> = attempted_models
        .iter()
        .map(|m| smart_switch_model_key(m))
        .collect();
    let candidates = smart_switch_candidates(
        current_model,
        current_provider,
        available_providers,
        &attempted,
    );
    let candidates = if is_provider_account_exhausted(error_msg) {
        filter_out_provider(candidates, current_provider)
    } else {
        candidates
    };
    candidates
        .into_iter()
        .next()
        .map(|target_model| super::pending::build(prompt, target_model))
}
