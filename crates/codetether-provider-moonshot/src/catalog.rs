//! Existing ordered Moonshot model metadata.
use crate::*;
pub(crate) fn models() -> Vec<ModelInfo> {
    vec![
        ModelInfo {
            id: "kimi-k2.5".to_string(),
            name: "Kimi K2.5".to_string(),
            provider: "moonshotai".to_string(),
            context_window: 256_000,
            max_output_tokens: Some(64_000),
            supports_vision: true,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(0.56), // ¥4/M tokens
            output_cost_per_million: Some(2.8), // ¥20/M tokens
        },
        ModelInfo {
            id: "kimi-k2-thinking".to_string(),
            name: "Kimi K2 Thinking".to_string(),
            provider: "moonshotai".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(64_000),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(0.56),
            output_cost_per_million: Some(2.8),
        },
        ModelInfo {
            id: "kimi-latest".to_string(),
            name: "Kimi Latest".to_string(),
            provider: "moonshotai".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(64_000),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(0.42), // Cheaper
            output_cost_per_million: Some(1.68),
        },
    ]
}
