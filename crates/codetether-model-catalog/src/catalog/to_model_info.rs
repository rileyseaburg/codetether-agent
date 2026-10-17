//! Catalog to model info.
use super::*;
impl ModelCatalog {
    /// Convert API model info to our internal ModelInfo format
    #[allow(dead_code)]
    pub fn to_model_info(
        &self,
        model: &ApiModelInfo,
        provider_id: &str,
    ) -> codetether_provider_core::ModelInfo {
        codetether_provider_core::ModelInfo {
            id: model.id.clone(),
            name: model.name.clone(),
            provider: provider_id.to_string(),
            context_window: model
                .limit
                .as_ref()
                .map(|l| l.context as usize)
                .unwrap_or(128_000),
            max_output_tokens: model.limit.as_ref().map(|l| l.output as usize),
            supports_vision: model.attachment,
            supports_tools: model.tool_call,
            supports_streaming: true,
            input_cost_per_million: model.cost.as_ref().map(|c| c.input),
            output_cost_per_million: model.cost.as_ref().map(|c| c.output),
        }
    }
}
