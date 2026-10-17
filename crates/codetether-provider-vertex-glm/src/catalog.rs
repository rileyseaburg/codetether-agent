//! Static Vertex GLM model metadata.
use crate::*;
pub(super) async fn list_models() -> Result<Vec<ModelInfo>> {
    Ok(vec![
        ModelInfo {
            id: "zai-org/glm-5-maas".to_string(),
            name: "GLM-5 (Vertex AI MaaS)".to_string(),
            provider: "vertex-glm".to_string(),
            context_window: 200_000,
            max_output_tokens: Some(128_000),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(1.0),
            output_cost_per_million: Some(3.2),
        },
        ModelInfo {
            id: "glm-5".to_string(),
            name: "GLM-5 (Vertex AI)".to_string(),
            provider: "vertex-glm".to_string(),
            context_window: 200_000,
            max_output_tokens: Some(128_000),
            supports_vision: false,
            supports_tools: true,
            supports_streaming: true,
            input_cost_per_million: Some(1.0),
            output_cost_per_million: Some(3.2),
        },
    ])
}
