//! Model metadata for gemini_3.
use crate::*;
pub(super) fn model_2() -> ModelInfo {
    ModelInfo {
        id: "gemini-3-pro-preview".to_string(),
        name: "Gemini 3 Pro Preview".to_string(),
        provider: "google".to_string(),
        context_window: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(2.0),
        output_cost_per_million: Some(12.0),
    }
}

pub(super) fn model_3() -> ModelInfo {
    ModelInfo {
        id: "gemini-3-flash-preview".to_string(),
        name: "Gemini 3 Flash Preview".to_string(),
        provider: "google".to_string(),
        context_window: 1_048_576,
        max_output_tokens: Some(65_536),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.50),
        output_cost_per_million: Some(3.0),
    }
}
