//! Flash model metadata.
use super::*;
pub(super) fn flash_model() -> ModelInfo {
    ModelInfo {
        id: "deepseek-v4-flash".into(),
        name: "DeepSeek V4 Flash".into(),
        provider: "deepseek".into(),
        context_window: 1_048_576,
        max_output_tokens: Some(393_216),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.14),
        output_cost_per_million: Some(0.28),
    }
}
