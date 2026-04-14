//! Model eligibility policy for sub-agent spawning.
//!
//! This module decides whether a model is safe to use for autonomous
//! spawned agents. It only checks eligibility and cost constraints,
//! and does not restrict parent/child model equality.
//!
//! # Examples
//!
//! ```ignore
//! let allowed = is_free_or_eligible("openai-codex/gpt-5-mini", &registry).await;
//! assert!(allowed);
//! ```

use super::policy_free;
use super::policy_parse;
use super::policy_registry;
use crate::provider::ProviderRegistry;

/// Returns whether a model is eligible for autonomous sub-agent use.
///
/// The policy allows free models, subscription-backed providers, and a small
/// explicit allowlist before consulting provider registry cost metadata.
///
/// # Examples
///
/// ```ignore
/// let allowed = is_free_or_eligible("openrouter/qwen/qwen3", &registry).await;
/// ```
pub(super) async fn is_free_or_eligible(model: &str, registry: &ProviderRegistry) -> bool {
    match policy_parse::parse_model_parts(model) {
        None => false,
        Some((None, raw_model)) => policy_free::is_free_model_id(raw_model),
        Some((Some(provider), model_id)) => {
            provider_model_is_eligible(provider, model_id, registry).await
        }
    }
}

async fn provider_model_is_eligible(
    provider: String,
    model_id: &str,
    registry: &ProviderRegistry,
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
