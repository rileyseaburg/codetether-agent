//! Cross-provider priority-ordered fallback candidates.

use std::collections::HashSet;

use super::error_detection::{normalize_provider_alias, smart_switch_model_key};
use super::models::smart_switch_preferred_models;
use crate::tui::constants::SMART_SWITCH_PROVIDER_PRIORITY;

/// Cross-provider fallbacks in priority order.
pub fn priority_candidates(
    current_provider: Option<&str>,
    available: &HashSet<String>,
    attempted_models: &HashSet<String>,
) -> Vec<String> {
    let mut out = Vec::new();
    for provider_name in SMART_SWITCH_PROVIDER_PRIORITY {
        let normalized = normalize_provider_alias(provider_name);
        if Some(normalized) == current_provider || !available.contains(normalized) {
            continue;
        }
        if let Some(model_id) = smart_switch_preferred_models(normalized).first() {
            let candidate = format!("{normalized}/{model_id}");
            if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                out.push(candidate);
            }
        }
    }
    out
}
