//! Legacy model metadata.
use super::*;
pub(super) fn chat_legacy() -> ModelInfo {
    ModelInfo {
        id: "deepseek-chat".into(),
        name: "DeepSeek Chat (legacy)".into(),
        provider: "deepseek".into(),
        context_window: 128_000,
        max_output_tokens: Some(128_000),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.14),
        output_cost_per_million: Some(0.28),
    }
}

pub(super) fn reasoner_legacy() -> ModelInfo {
    ModelInfo {
        id: "deepseek-reasoner".into(),
        name: "DeepSeek Reasoner (legacy)".into(),
        provider: "deepseek".into(),
        context_window: 128_000,
        max_output_tokens: Some(128_000),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.14),
        output_cost_per_million: Some(0.28),
    }
}
