//! Candidate generation for smart switching — combines provider strategies.

use std::collections::HashSet;

use super::error_detection::normalize_provider_alias;
use super::extra::extra_candidates;
use super::priority::priority_candidates;
use super::same_provider::same_provider_candidates;

/// Generates candidate models for smart switching, prioritizing same-provider.
pub fn smart_switch_candidates(
    _current_model: Option<&str>,
    current_provider: Option<&str>,
    available_providers: &[String],
    attempted_models: &HashSet<String>,
) -> Vec<String> {
    let available: HashSet<String> = available_providers
        .iter()
        .map(|p| normalize_provider_alias(p).to_string())
        .collect();
    let current_normalized = current_provider.map(normalize_provider_alias);
    let mut candidates = Vec::new();
    if let Some(p) = current_normalized {
        candidates.extend(same_provider_candidates(p, &available, attempted_models));
    }
    candidates.extend(priority_candidates(
        current_normalized,
        &available,
        attempted_models,
    ));
    candidates.extend(extra_candidates(
        current_normalized,
        &available,
        attempted_models,
    ));
    candidates
}
