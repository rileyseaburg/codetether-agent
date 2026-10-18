use codetether_provider_core::ModelInfo;

pub(super) fn model(id: &str, cost: Option<f64>, tools: bool) -> ModelInfo {
    ModelInfo {
        id: id.into(),
        name: id.into(),
        provider: "same".into(),
        context_window: 128_000,
        max_output_tokens: Some(8_192),
        supports_vision: false,
        supports_tools: tools,
        supports_streaming: true,
        input_cost_per_million: cost,
        output_cost_per_million: None,
    }
}
