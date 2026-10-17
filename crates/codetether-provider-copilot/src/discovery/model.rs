//! Discovered model metadata projection.
use crate::*;
impl CopilotProvider {
    pub(super) fn discovery_model(&self, model: CopilotModelInfo) -> ModelInfo {
        let caps = model.capabilities.as_ref();
        let limits = caps.and_then(|c| c.limits.as_ref());
        let supports = caps.and_then(|c| c.supports.as_ref());

        ModelInfo {
            id: model.id.clone(),
            name: model.name.unwrap_or_else(|| model.id.clone()),
            provider: self.provider_name.clone(),
            context_window: limits
                .and_then(|l| l.max_context_window_tokens)
                .unwrap_or(128_000),
            max_output_tokens: limits.and_then(|l| l.max_output_tokens).or(Some(16_384)),
            supports_vision: supports.and_then(|s| s.vision).unwrap_or(false),
            supports_tools: supports.and_then(|s| s.tool_calls).unwrap_or(true),
            supports_streaming: supports.and_then(|s| s.streaming).unwrap_or(true),
            input_cost_per_million: None,
            output_cost_per_million: None,
        }
    }
}
