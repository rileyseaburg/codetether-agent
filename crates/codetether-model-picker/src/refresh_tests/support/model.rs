//! Model descriptors shared by the refresh test providers.
use codetether_provider_core::ModelInfo;
pub(in crate::refresh_tests) fn model(id: &str, provider: &str) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: id.to_string(),
        provider: provider.to_string(),
        context_window: 128_000,
        max_output_tokens: Some(16_384),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    }
}
