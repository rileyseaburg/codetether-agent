//! Codex available models.
use super::*;

/// Return the ordered offline selector list for the selected backend.
///
/// # Arguments
/// `chatgpt_backend` selects the ChatGPT filter instead of API-key filtering.
/// # Returns
/// Filtered seed metadata in its original order.
/// # Examples
/// ```
/// assert!(!codetether_provider_core::codex::model_policy::available_models(false).is_empty());
/// ```
pub fn available_models(chatgpt_backend: bool) -> Vec<ModelInfo> {
    let mut models = vec![
        astra_model::info("gpt-6-astra", "GPT-6 Astra"),
        astra_model::info("gpt-6-astra-fast", "GPT-6 Astra Fast"),
        model_info("gpt-5.5", "GPT-5.5", 272_000, 128_000, false),
        model_info("gpt-5.5-fast", "GPT-5.5 Fast", 272_000, 128_000, false),
        model_info("gpt-5.6-sol", "GPT-5.6 Sol", 272_000, 128_000, false),
        model_info("gpt-5.6-terra", "GPT-5.6 Terra", 272_000, 128_000, false),
        model_info("gpt-5.6-luna", "GPT-5.6 Luna", 272_000, 128_000, false),
        model_info("gpt-reserve", "GPT-Reserve", 272_000, 128_000, false),
        model_info("gpt-5.4", "GPT-5.4", 272_000, 128_000, false),
        model_info("gpt-5.4-mini", "GPT-5.4 Mini", 272_000, 128_000, false),
        model_info(
            "codex-auto-review",
            "Codex Auto Review",
            272_000,
            128_000,
            false,
        ),
    ];
    if chatgpt_backend {
        models.retain(|model| model_catalog::chatgpt_models().contains(&model.id.as_str()));
    } else {
        models.retain(|model| !api_key_hidden_model(&model.id));
    }
    models
}
