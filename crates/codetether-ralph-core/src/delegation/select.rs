//! Choose a provider using delegated role policy and lazy candidates.
use super::*;
/// Rank provider names for a role, retaining defaults when delegation is disabled.
///
/// # Arguments
/// `state` supplies ranking policy; `role` supplies its skill context.
/// `default_provider` and `default_model` are fallback values; `providers`
/// lists candidates only after the enabled-policy check and role mapping.
/// # Returns
/// Owned provider and model names; selection retains the default model.
/// # Examples
/// ```rust,no_run
/// fn example(state: &codetether_context_policy::delegation::DelegationState) {
/// let selected = codetether_ralph_core::delegation::choose_provider_for_role(
///     state, "auto-coder", "default", "model", || vec!["default"]);
/// assert_eq!(selected.1, "model");
/// }
/// ```

/// Pick the best `(provider_name, model_name)` for a relay role.
///
/// Maps the role to a relevance bucket and delegates to
/// [`DelegationState::rank_candidates`].
pub fn choose_provider_for_role<'a>(
    state: &DelegationState,
    role: &str,
    default_provider: &str,
    default_model: &str,
    providers: impl FnOnce() -> Vec<&'a str>,
) -> (String, String) {
    if !state.enabled() {
        return (default_provider.to_string(), default_model.to_string());
    }
    let skill = skill_for_role(role);
    let bucket = Bucket {
        difficulty: difficulty_for_role(role),
        dependency: Dependency::Isolated,
        tool_use: tool_use_for_role(role),
    };
    let providers = providers();
    let picked = state.rank_candidates(&providers, skill, bucket);
    match picked {
        Some(p) => {
            tracing::info!(target: "codetether_agent::ralph::delegation", provider = %p, role = %role, "LCB selected provider for relay role");
            (p.to_string(), default_model.to_string())
        }
        None => (default_provider.to_string(), default_model.to_string()),
    }
}
