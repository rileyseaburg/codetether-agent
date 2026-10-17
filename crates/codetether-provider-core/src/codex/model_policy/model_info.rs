//! Codex model info.
use super::*;

/// Construct Codex selector metadata with the provided capability limits.
///
/// # Arguments
/// `id` and `name` identify the model; token limits and vision flag describe it.
/// # Returns
/// Codex metadata with tool/stream support and the existing zero-price defaults.
/// # Examples
/// ```
/// let model = codetether_provider_core::codex::model_policy::model_info("example", "Example", 100, 20, false);
/// assert_eq!(model.max_output_tokens, Some(20));
/// ```
pub fn model_info(
    id: &str,
    name: &str,
    context_window: usize,
    max_output_tokens: usize,
    supports_vision: bool,
) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: name.to_string(),
        provider: "openai-codex".to_string(),
        context_window,
        max_output_tokens: Some(max_output_tokens),
        supports_vision,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.0),
        output_cost_per_million: Some(0.0),
    }
}
