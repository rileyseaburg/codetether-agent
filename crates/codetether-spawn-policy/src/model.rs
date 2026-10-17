//! Existing subscription, free-tier, allowlist and catalog eligibility rules.
use super::policy_free;
use super::policy_parse;
use super::policy_registry;
use crate::ModelRegistry;

/// Determine whether a model is eligible under the existing spawn cost policy.
/// # Arguments
/// * `model` — Provider/model reference, or an unqualified free-tier model ID.
/// * `registry` — Existing provider handles for catalog cost inspection.
/// # Returns
/// True for existing subscription/free/allowlist rules or zero-cost catalog entries.
/// Failed or missing catalog lookups return false.
/// # Examples
/// ```rust,no_run
/// use codetether_spawn_policy::{is_free_or_eligible, ModelRegistry};
/// async fn example<R: ModelRegistry>(registry: &R) {
///     assert!(is_free_or_eligible("openai-codex/model", registry).await);
/// }
/// ```
pub async fn is_free_or_eligible<R: ModelRegistry>(model: &str, registry: &R) -> bool {
    match policy_parse::parse_model_parts(model) {
        None => false,
        Some((None, raw_model)) => policy_free::is_free_model_id(raw_model),
        Some((Some(provider), model_id)) => {
            provider_model_is_eligible(provider, model_id, registry).await
        }
    }
}

async fn provider_model_is_eligible<R: ModelRegistry>(
    provider: String,
    model_id: &str,
    registry: &R,
) -> bool {
    if policy_free::is_subscription_provider(&provider.to_ascii_lowercase())
        || policy_free::is_free_model_id(model_id)
        || policy_free::is_budget_allowlisted_model(&provider, model_id)
    {
        return true;
    }
    policy_registry::registry_model_is_free(&provider, model_id, registry).await
}

#[cfg(test)]
pub(super) use policy_free::{is_budget_allowlisted_model, is_free_model_id};
#[cfg(test)]
pub(super) use policy_parse::normalize_model;

#[cfg(test)]
#[path = "policy_tests.rs"]
mod tests;
