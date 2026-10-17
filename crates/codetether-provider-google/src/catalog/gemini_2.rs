//! Model metadata for gemini_2.
use crate::*;
pub(super) fn model_7() -> ModelInfo {
    ModelInfo {
        id: "gemini-2.0-flash".to_string(),
        name: "Gemini 2.0 Flash".to_string(),
        provider: "google".to_string(),
        context_window: 1_048_576,
        max_output_tokens: Some(8_192),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.10),
        output_cost_per_million: Some(0.40),
    }
}
