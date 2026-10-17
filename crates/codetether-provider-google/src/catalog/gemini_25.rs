//! Model metadata for gemini_25.
use crate::*;
pub(super) fn model_5() -> ModelInfo {
    ModelInfo {
        id: "gemini-2.5-pro".to_string(),
        name: "Gemini 2.5 Pro".to_string(),
        provider: "google".to_string(),
        context_window: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(1.25),
        output_cost_per_million: Some(10.0),
    }
}

pub(super) fn model_6() -> ModelInfo {
    ModelInfo {
        id: "gemini-2.5-flash".to_string(),
        name: "Gemini 2.5 Flash".to_string(),
        provider: "google".to_string(),
        context_window: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.15),
        output_cost_per_million: Some(0.60),
    }
}
