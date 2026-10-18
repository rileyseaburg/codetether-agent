//! Ordered and deduplicated failover candidate assembly.
use super::{preferences::preferred_failover_model_refs, provider_candidates as router_candidates};

/// Build ordered failover references, excluding the failed model and duplicates.
/// # Arguments
/// * `provider` — Current provider used for the same-provider candidate ladder.
/// * `failed_model` — Model reference or identifier that must not be retried.
/// # Returns
/// Configured preferences, provider siblings, and cross-provider fallback refs.
/// # Examples
/// ```
/// let refs = codetether_model_failover::router::known_good_router_candidates("unknown", "failed");
/// assert!(refs.iter().any(|model| model == "zai/glm-5"));
/// ```
pub fn known_good_router_candidates(provider: &str, failed_model: &str) -> Vec<String> {
    let failed = failed_model.trim();
    let mut candidates: Vec<String> = preferred_failover_model_refs();

    candidates.extend(router_candidates::provider_failover_candidates(provider));

    candidates.extend([
        "minimax/MiniMax-M3".to_string(),
        "zai/glm-5.1".to_string(),
        "zai/glm-5".to_string(),
    ]);

    let mut seen = std::collections::HashSet::new();
    candidates.retain(|candidate| {
        let unique = seen.insert(candidate.to_ascii_lowercase());
        unique
            && !candidate.eq_ignore_ascii_case(failed)
            && candidate
                .split('/')
                .next_back()
                .map(|model| !model.eq_ignore_ascii_case(failed))
                .unwrap_or(true)
    });
    candidates
}
