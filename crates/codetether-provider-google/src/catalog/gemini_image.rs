//! Model metadata for gemini_image.
use crate::*;
pub(super) fn model_4() -> ModelInfo {
    ModelInfo {
        id: "gemini-3-pro-image-preview".to_string(),
        name: "Gemini 3 Pro Image Preview".to_string(),
        provider: "google".to_string(),
        context_window: 65_536,
        max_output_tokens: Some(32_768),
        supports_vision: true,
        supports_tools: false,
        supports_streaming: false,
        input_cost_per_million: Some(2.0),
        output_cost_per_million: Some(134.0),
    }
}
