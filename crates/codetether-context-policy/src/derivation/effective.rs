//! Effective context policy from persisted state and environment overrides.
use super::{DerivePolicy, incremental_budget_config, policy_implicit};
use crate::derivation::reset::markers::latest_reset_marker_index;
use codetether_provider_core::Message;
use std::env;
#[path = "effective_threshold.rs"]
mod threshold;
use threshold::resolve_reset_threshold;
/// Resolve implicit policy and process-wide overrides.
///
/// # Arguments
/// * `persisted` - Session policy selected before this request.
/// * `messages` - Current immutable transcript used for reset-marker detection.
/// * `model` - Model whose capacity determines budgets.
/// * `warn_unknown` - Host warning hook, called only for an unknown override.
///
/// # Returns
/// The effective policy with resolved model-relative budgets.
///
/// # Examples
/// ```no_run
/// use codetether_context_policy::derivation::{DerivePolicy, effective::effective_policy};
/// let policy = effective_policy(DerivePolicy::default(), &[], "gpt-4o", |_| {});
/// assert!(["legacy", "reset", "incremental", "oracle_replay"].contains(&policy.kind()));
/// ```
pub fn effective_policy(
    persisted: DerivePolicy,
    messages: &[Message],
    model: &str,
    warn_unknown: impl FnOnce(&str),
) -> DerivePolicy {
    let implicit = policy_implicit::resolve(policy_implicit::Inputs {
        persisted,
        has_reset_marker: latest_reset_marker_index(messages).is_some(),
        reset_threshold: resolve_reset_threshold(persisted, model),
        incremental_budget: incremental_budget_config::resolve(persisted, model),
    });
    let Ok(raw) = env::var("CODETETHER_CONTEXT_POLICY") else {
        return implicit;
    };
    match raw.trim().to_ascii_lowercase().as_str() {
        "" => implicit,
        "legacy" => DerivePolicy::Legacy,
        "reset" => DerivePolicy::Reset {
            threshold_tokens: resolve_reset_threshold(persisted, model),
        },
        "incremental" => DerivePolicy::Incremental {
            budget_tokens: super::incremental_budget_config::resolve(persisted, model),
        },
        _ => {
            warn_unknown(&raw);
            implicit
        }
    }
}
