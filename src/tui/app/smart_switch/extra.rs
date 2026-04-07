//! Extra (non-priority) provider fallback candidates.

use std::collections::HashSet;

use super::error_detection::{normalize_provider_alias, smart_switch_model_key};
use super::models::smart_switch_preferred_models;
use crate::tui::constants::SMART_SWITCH_PROVIDER_PRIORITY;

/// Remaining providers not in the priority list.
pub fn extra_candidates(
    current_provider: Option<&str>,
    available: &HashSet<String>,
    attempted_models: &HashSet<String>,
) -> Vec<String> {
    let mut extra: Vec<String> = available
        .iter()
        .filter(|p| {
            Some(p.as_str()) != current_provider
                && !SMART_SWITCH_PROVIDER_PRIORITY
                    .iter()
                    .any(|pr| normalize_provider_alias(pr) == *p)
        })
        .cloned()
        .collect();
    extra.sort();
    let mut out = Vec::new();
    for provider_name in extra {
        if let Some(model_id) = smart_switch_preferred_models(&provider_name).first() {
            let candidate = format!("{provider_name}/{model_id}");
            if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                out.push(candidate);
            }
        }
    }
    out
}
