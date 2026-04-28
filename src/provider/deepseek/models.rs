//! Model list for the DeepSeek provider.

use crate::provider::ModelInfo;

pub(crate) fn list() -> Vec<ModelInfo> {
    vec![flash_model(), pro_model(), chat_legacy(), reasoner_legacy()]
}

fn flash_model() -> ModelInfo {
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

fn pro_model() -> ModelInfo {
    ModelInfo {
        id: "deepseek-v4-pro".into(),
        name: "DeepSeek V4 Pro".into(),
        provider: "deepseek".into(),
        context_window: 1_048_576,
        max_output_tokens: Some(393_216),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.435),
        output_cost_per_million: Some(0.87),
    }
}

fn chat_legacy() -> ModelInfo {
    ModelInfo {
        id: "deepseek-chat".into(),
        name: "DeepSeek Chat (legacy)".into(),
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

fn reasoner_legacy() -> ModelInfo {
    ModelInfo {
        id: "deepseek-reasoner".into(),
        name: "DeepSeek Reasoner (legacy)".into(),
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
