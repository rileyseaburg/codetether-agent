//! Delegation-score ordering of failover candidates.
use super::{choose_router_target, known_good_router_candidates};

/// Rank failover targets with delegation scores, retaining rule order for ties.
/// Falls back to rule-based selection if scoring is disabled or no scored target is available.
/// # Arguments
/// * `registry` — Active provider availability.
/// * `state`, `bucket` — Delegation scores and current request context.
/// * `selected_provider`, `current_model` — Current failed model identity.
/// # Returns
/// The highest-ranked available candidate, or the ordinary router fallback.
/// # Examples
/// ```no_run
/// # fn select(registry: &impl codetether_model_failover::router::Registry, state: &codetether_context_policy::delegation::DelegationState, bucket: codetether_context_policy::relevance::Bucket) {
/// let _target = codetether_model_failover::router::choose_router_target_bandit(registry, state, bucket, "openai", "failed");
/// # }
/// ```
pub fn choose_router_target_bandit(
    registry: &impl super::Registry,
    state: &codetether_context_policy::delegation::DelegationState,
    bucket: codetether_context_policy::relevance::Bucket,
    selected_provider: &str,
    current_model: &str,
) -> Option<(String, String)> {
    if !state.enabled() {
        return choose_router_target(registry, selected_provider, current_model);
    }

    // Enumerate rule-based candidates, annotate each with an LCB score
    // (or 0.0 when there is no posterior yet), sort stably by score
    // descending, and return the first one the registry knows about.
    let candidates = known_good_router_candidates(selected_provider, current_model);
    let mut scored: Vec<(String, f64)> = candidates
        .into_iter()
        .map(|raw| {
            let (provider_name, _model_name) =
                codetether_provider_core::routing::parse_model_string(&raw);
            let provider_name = provider_name.unwrap_or(selected_provider);
            let score = state
                .score(
                    provider_name,
                    codetether_context_policy::delegation::skills::MODEL_CALL,
                    bucket,
                )
                .unwrap_or(0.0);
            (raw, score)
        })
        .collect();
    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    for (candidate, _score) in scored {
        let (provider_name, model_name) =
            codetether_provider_core::routing::parse_model_string(&candidate);
        let provider_name = provider_name.unwrap_or(selected_provider);
        if registry.available(provider_name) {
            return Some((provider_name.to_string(), model_name.to_string()));
        }
    }

    // No re-ordered candidate was registered — fall back to the
    // original rule-based ladder so we never regress against the
    // non-bandit behaviour.
    choose_router_target(registry, selected_provider, current_model)
}
