//! Small helper functions for smart switch.

use crate::tui::constants::SMART_SWITCH_MAX_RETRIES;

/// Returns the max number of smart switch retries (from env or default).
pub fn smart_switch_max_retries() -> u8 {
    std::env::var("CODETETHER_SMART_SWITCH_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .map(|v| v.clamp(1, 10))
        .unwrap_or(SMART_SWITCH_MAX_RETRIES)
}

/// Extracts provider name from a model reference (e.g. "minimax/MiniMax-M2" -> "minimax").
pub fn extract_provider_from_model(model_ref: &str) -> Option<&str> {
    model_ref.split('/').next().filter(|p| !p.is_empty())
}

/// Returns true when the model changed compared to the current session model.
pub fn should_execute_smart_switch(
    current_model: Option<&str>,
    pending: Option<&super::retry::PendingSmartSwitchRetry>,
) -> bool {
    match (current_model, pending) {
        (Some(current), Some(retry)) => current != retry.target_model,
        (_, Some(_)) => true,
        _ => false,
    }
}
