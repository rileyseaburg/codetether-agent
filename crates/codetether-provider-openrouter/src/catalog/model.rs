//! Catalog capability mapping.
use super::wire::ModelData;
use crate::*;
pub(super) fn info(m: ModelData) -> ModelInfo {
    ModelInfo {
        id: m.id.clone(),
        name: m.name.unwrap_or_else(|| m.id.clone()),
        provider: "openrouter".to_string(),
        context_window: m.context_length.unwrap_or(128_000),
        max_output_tokens: Some(16_384),
        supports_vision: false,
        supports_tools: model_capabilities::catalog_supports_tools(&m.id, &m.supported_parameters),
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    }
}
