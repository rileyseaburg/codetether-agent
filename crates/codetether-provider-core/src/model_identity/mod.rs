//! Canonical provider model identities, shared with request construction.

pub mod bedrock;
pub mod cerebras;
pub mod glm5;
pub mod vertex_glm;

/// Normalize a model using the selected provider's request-alias rules.
///
/// # Arguments
/// * `provider` - Provider identifier.
/// * `model` - User-selected model or alias.
/// # Returns
/// The upstream model identifier; unknown providers preserve the input.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_core::model_identity::resolve("cerebras", "glm-4.7"), "zai-glm-4.7");
/// ```
pub fn resolve(provider: &str, model: &str) -> String {
    match provider {
        "bedrock" => bedrock::resolve_model_id(model).to_owned(),
        "openai-codex" => crate::codex::resolved_model_identity(model),
        "cerebras" => cerebras::normalize_model_id(provider, model).into_owned(),
        "glm5" => glm5::normalize_model(model),
        "vertex-glm" => vertex_glm::normalize(model),
        _ => model.to_owned(),
    }
}
