//! Provider ranking with lazy registry enumeration and preserved fallback.
use crate::bucket_for_specialty;
use codetether_context_policy::delegation::{DelegationState, skills as delegation_skills};
/// Pick the best `(provider_name, model_name)` for a subtask.
///
/// Returns the LCB-optimal provider when delegation is enabled,
/// or the supplied default when it is not.
/// # Arguments
/// `providers` lazily lists available providers only when delegation is enabled.
/// The remaining arguments supply calibration state, specialty and fallback selection.
/// # Returns
/// The selected provider and the unchanged default model.
/// # Examples
/// ```no_run
/// fn choose(state: &codetether_context_policy::delegation::DelegationState) {
///     let _ = codetether_swarm_delegation::choose_provider_for_subtask(|| vec!["one"], state, "review", "one", "model");
/// }
/// ```
pub fn choose_provider_for_subtask<'a>(
    providers: impl FnOnce() -> Vec<&'a str>,
    state: &DelegationState,
    specialty: &str,
    default_provider: &str,
    default_model: &str,
) -> (String, String) {
    if !state.enabled() {
        return (default_provider.to_string(), default_model.to_string());
    }
    let bucket = bucket_for_specialty(specialty);
    let providers = providers();
    let picked = state.rank_candidates(&providers, delegation_skills::SWARM_DISPATCH, bucket);
    match picked {
        Some(p) => {
            tracing::info!(provider = %p, specialty = %specialty, "LCB selected provider");
            (p.to_string(), default_model.to_string())
        }
        None => (default_provider.to_string(), default_model.to_string()),
    }
}
