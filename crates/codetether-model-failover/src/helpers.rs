//! Small helper functions for smart switch.

use crate::constants::SMART_SWITCH_MAX_RETRIES;

/// Returns the max number of smart switch retries (from env or default).
///
/// # Returns
/// The normalized value or policy result described above.
///
/// # Examples
/// ```
/// assert!((1..=10).contains(&codetether_model_failover::smart_switch_max_retries()));
/// ```
pub fn smart_switch_max_retries() -> u8 {
    std::env::var("CODETETHER_SMART_SWITCH_MAX_RETRIES")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .map(|v| v.clamp(1, 10))
        .unwrap_or(SMART_SWITCH_MAX_RETRIES)
}

/// Extracts provider name from a model reference (e.g. "minimax/MiniMax-M2" -> "minimax").
///
/// # Arguments
/// * `model_ref` — model ref input.
///
/// # Returns
/// The selected value when the policy permits it, otherwise None.
///
/// # Examples
/// ```
/// assert_eq!(codetether_model_failover::helpers::extract_provider_from_model("zai/glm-5"), Some("zai"));
/// ```
pub fn extract_provider_from_model(model_ref: &str) -> Option<&str> {
    model_ref.split('/').next().filter(|p| !p.is_empty())
}

/// Returns true when the model changed compared to the current session model.
///
/// # Arguments
/// * `current_model` — current model input.
/// * `pending` — pending input.
///
/// # Returns
/// Whether the documented policy condition matches.
///
/// # Examples
/// ```
/// assert!(!codetether_model_failover::should_execute_smart_switch(None, None));
/// ```
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
