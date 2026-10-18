//! RLM CLI provider selection with caller-owned provider lookup.
//!
//! resolve preserves explicit provider selection, configured local-CUDA
//! precedence and OpenRouter fallback. It never initializes providers.
mod aliases;
mod default_selection;
mod environment;
mod explicit;
mod unqualified;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Caller lookup of an already-initialized provider by canonical name.
pub type ProviderLookup<'a> = dyn Fn(&str) -> Option<Arc<dyn Provider>> + 'a;
/// Selected provider, provider-local model ID and canonical provider name.
pub type SelectedProvider = (Arc<dyn Provider>, String, String);
/// Select the RLM provider using the existing CLI policy.
/// # Arguments
/// * `model_arg` - Optional explicit model or provider/model reference.
/// * `lookup` - Lookup against caller-owned initialized providers.
/// # Returns
/// Provider instance, model identifier and canonical provider name.
/// # Errors
/// Returns existing configuration/availability errors when no candidate works.
/// # Examples
/// ```no_run
/// let lookup = |_name: &str| None;
/// let _ = codetether_rlm_selection::resolve(Some("openrouter/model"), &lookup);
/// ```
pub fn resolve(
    model_arg: Option<&str>,
    lookup: &ProviderLookup<'_>,
) -> anyhow::Result<SelectedProvider> {
    if let Some(model_ref) = model_arg {
        let (provider_name, model_name) =
            codetether_provider_core::routing::parse_model_string(model_ref);
        if let Some(provider_name) = provider_name {
            return explicit::select(provider_name, model_name, lookup);
        }
        return unqualified::select(model_ref, lookup);
    }
    default_selection::select(lookup)
}
