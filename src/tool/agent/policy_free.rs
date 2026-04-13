//! Free-tier and allowlist checks for agent spawn policy.
//!
//! This module groups pure helper predicates used by the spawned-agent
//! eligibility policy.
//!
//! # Examples
//!
//! ```ignore
//! assert!(is_free_model_id("model:free"));
//! ```

use super::policy_constants::{OPENROUTER_BUDGET_ALLOWLIST, SUBSCRIPTION_PROVIDERS};

/// Returns whether a provider is subscription-backed rather than token-billed.
///
/// # Examples
///
/// ```ignore
/// assert!(is_subscription_provider("zai"));
/// ```
pub(super) fn is_subscription_provider(provider: &str) -> bool {
    SUBSCRIPTION_PROVIDERS.contains(&provider)
}

/// Returns whether a model id clearly marks itself as free-tier.
///
/// # Examples
///
/// ```ignore
/// assert!(is_free_model_id("model:free"));
/// ```
pub(super) fn is_free_model_id(id: &str) -> bool {
    let lower = id.to_ascii_lowercase();
    lower.contains(":free") || lower.ends_with("-free")
}

/// Returns whether a provider/model pair is explicitly budget-allowlisted.
///
/// # Examples
///
/// ```ignore
/// let allowed = is_budget_allowlisted_model("openrouter", "google/gemini-2.5-flash-lite");
/// ```
pub(super) fn is_budget_allowlisted_model(provider: &str, model_id: &str) -> bool {
    provider.eq_ignore_ascii_case("openrouter")
        && OPENROUTER_BUDGET_ALLOWLIST
            .iter()
            .any(|allowed| allowed.eq_ignore_ascii_case(model_id.trim()))
}
