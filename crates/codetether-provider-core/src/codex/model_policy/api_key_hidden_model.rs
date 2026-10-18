//! Codex api key hidden model.

/// Check whether a seed model is hidden in API-key mode.
///
/// # Arguments
/// `model` is a model identifier without request options.
/// # Returns
/// Whether the existing ChatGPT-only seed filter excludes the model.
/// # Examples
/// ```
/// assert!(codetether_provider_core::codex::model_policy::api_key_hidden_model("gpt-reserve"));
/// ```
pub fn api_key_hidden_model(model: &str) -> bool {
    matches!(
        model,
        "gpt-5.5"
            | "gpt-5.5-fast"
            | "gpt-5.6-sol"
            | "gpt-5.6-terra"
            | "gpt-5.6-luna"
            | "gpt-reserve"
            | "codex-auto-review"
    )
}
