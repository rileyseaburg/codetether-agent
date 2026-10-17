//! Existing model selection regression coverage.
use super::*;

pub(super) fn model(id: &str) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: id.to_string(),
        provider: "test".to_string(),
        context_window: 128_000,
        max_output_tokens: Some(16_384),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.0),
        output_cost_per_million: Some(0.0),
    }
}
