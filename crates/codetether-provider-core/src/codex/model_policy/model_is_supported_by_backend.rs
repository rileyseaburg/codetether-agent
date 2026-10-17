//! Codex model is supported by backend.
use super::*;

/// Whether `model` is known for the active backend.
///
/// API-key mode accepts every model. On the ChatGPT backend a model is
/// known when it appears in the latest live discovery or the offline seed.
/// Check model eligibility for the active backend.
///
/// # Arguments
/// `chatgpt_backend` selects eligibility policy; `model` is the resolved identifier.
/// # Returns
/// True for API-key mode, or a model found in live discovery or the offline seed.
/// # Examples
/// ```
/// assert!(codetether_provider_core::codex::model_policy::model_is_supported_by_backend(false, "custom"));
/// ```
pub fn model_is_supported_by_backend(chatgpt_backend: bool, model: &str) -> bool {
    if !chatgpt_backend {
        return true;
    }
    model_discovery::discovered_slugs().is_some_and(|slugs| slugs.iter().any(|s| s == model))
        || model_catalog::chatgpt_models().contains(&model)
}
