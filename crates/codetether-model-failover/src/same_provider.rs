//! Same-provider fallback generation.

use std::collections::HashSet;

use super::error_detection::smart_switch_model_key;
use super::models::smart_switch_preferred_models;

/// Returns candidates from the same provider.
///
/// # Arguments
/// * `provider_name` — provider name input.
/// * `available` — available input.
/// * `attempted_models` — attempted models input.
///
/// # Returns
/// Ordered candidates remaining after policy filtering.
///
/// # Examples
/// ```
/// let empty = std::collections::HashSet::new();
/// assert!(codetether_model_failover::same_provider::same_provider_candidates("zai", &empty, &empty).is_empty());
/// ```
pub fn same_provider_candidates(
    provider_name: &str,
    available: &HashSet<String>,
    attempted_models: &HashSet<String>,
) -> Vec<String> {
    let mut out = Vec::new();
    if available.contains(provider_name) {
        for model_id in smart_switch_preferred_models(provider_name) {
            let candidate = format!("{provider_name}/{model_id}");
            if !attempted_models.contains(&smart_switch_model_key(&candidate)) {
                out.push(candidate);
            }
        }
    }
    out
}
