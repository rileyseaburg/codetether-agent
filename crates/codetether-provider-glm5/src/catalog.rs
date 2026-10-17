//! Static endpoint model metadata.
use crate::*;
pub(crate) async fn list_models() -> Result<Vec<ModelInfo>> {
    Ok(vec![
        ModelInfo {
            id: "glm-5-fp8".to_string(),
            name: "GLM-5 FP8 (744B MoE, 40B active)".to_string(),
            provider: "glm5".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(16_384),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        },
        ModelInfo {
            id: "glm-5".to_string(),
            name: "GLM-5 (alias → FP8)".to_string(),
            provider: "glm5".to_string(),
            context_window: 128_000,
            max_output_tokens: Some(16_384),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: None,
            output_cost_per_million: None,
        },
    ])
}
