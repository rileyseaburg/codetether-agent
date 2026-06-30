//! Default model catalog for the first-party OpenAI provider.

use crate::provider::ModelInfo;

/// Built-in OpenAI default models (used when the live list is unavailable).
pub(super) fn openai_default_models() -> Vec<ModelInfo> {
    vec![
        model("gpt-4o", "GPT-4o", 128_000, 16_384, 2.5, 10.0),
        model("gpt-4o-mini", "GPT-4o Mini", 128_000, 16_384, 0.15, 0.6),
        model("o1", "o1", 200_000, 100_000, 15.0, 60.0),
    ]
}

fn model(
    id: &str,
    name: &str,
    context_window: usize,
    max_output: usize,
    input_cost: f64,
    output_cost: f64,
) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: name.to_string(),
        provider: "openai".to_string(),
        context_window,
        max_output_tokens: Some(max_output),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(input_cost),
        output_cost_per_million: Some(output_cost),
    }
}
