//! Codex model supports reasoning effort.

/// Check the existing model-family rule for reasoning effort.
///
/// # Arguments
/// `model` is a resolved model identifier.
/// # Returns
/// Whether the model name matches a reasoning-capable family.
/// # Examples
/// ```
/// assert!(codetether_provider_core::codex::model_policy::model_supports_reasoning_effort("gpt-5.5"));
/// ```
pub fn model_supports_reasoning_effort(model: &str) -> bool {
    let normalized = model.to_ascii_lowercase();
    normalized.starts_with("gpt-6")
        || normalized.starts_with("gpt-5")
        || normalized.starts_with("openai.gpt-6")
        || normalized.starts_with("openai.gpt-5")
        || normalized.starts_with("o1")
        || normalized.starts_with("o3")
        || normalized.starts_with("o4")
}
