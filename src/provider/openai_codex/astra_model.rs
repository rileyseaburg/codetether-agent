//! Selector metadata for the announced GPT-6 Astra API model.
//!
//! OpenAI announced the API ID, pricing, vision/computer-use support, and
//! long-context results up to one million tokens. Until live model metadata
//! reaches this account, the output envelope stays at a conservative 128k.

use crate::provider::ModelInfo;

const CONTEXT_WINDOW: usize = 1_000_000;
const MAX_OUTPUT_TOKENS: usize = 128_000;
const INPUT_COST_PER_MILLION: f64 = 10.0;
const OUTPUT_COST_PER_MILLION: f64 = 50.0;

/// Build selector metadata for an Astra service-tier alias.
///
/// # Arguments
///
/// * `id` — Wire model ID, such as `gpt-6-astra`.
/// * `name` — Human-readable selector label.
///
/// # Returns
///
/// Model capabilities and announced standard API pricing.
///
/// # Examples
///
/// ```ignore
/// let model = info("gpt-6-astra", "GPT-6 Astra");
/// assert_eq!(model.context_window, 1_000_000);
/// ```
pub(super) fn info(id: &str, name: &str) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: name.to_string(),
        provider: "openai-codex".to_string(),
        context_window: CONTEXT_WINDOW,
        max_output_tokens: Some(MAX_OUTPUT_TOKENS),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(INPUT_COST_PER_MILLION),
        output_cost_per_million: Some(OUTPUT_COST_PER_MILLION),
    }
}