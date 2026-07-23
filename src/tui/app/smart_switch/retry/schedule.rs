//! Candidate selection for a provider-error smart retry.

use std::collections::HashSet;

use super::pending::PendingSmartSwitchRetry;
use crate::tui::app::smart_switch::account_state::is_provider_account_exhausted;
use crate::tui::app::smart_switch::candidates::smart_switch_candidates;
use crate::tui::app::smart_switch::error_detection::{
    is_retryable_provider_error, smart_switch_model_key,
};
use crate::tui::app::smart_switch::provider_filter::filter_out_provider;

/// Schedules a smart-switch retry if the error is retryable and candidates exist.
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
        .map(|target_model| PendingSmartSwitchRetry {
            prompt: prompt.to_string(),
            target_model,
            not_before: None,
            restore_from: None,
            restore_at: None,
            restore_model: None,
        })
}
