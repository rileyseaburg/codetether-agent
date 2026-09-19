//! Native provider capability catalog; Bonsai uses its conservative runtime limit.
use super::*;
impl LocalCudaProvider {
    pub(super) async fn model_catalog(&self) -> Result<Vec<ModelInfo>> {
        // Note: streaming and tool support are planned features, currently unimplemented
        Ok(vec![ModelInfo {
            id: self.model_name.clone(),
            name: self.model_name.clone(),
            provider: "local_cuda".to_string(),
            context_window: if self
                .resolve_architecture()
                .as_deref()
                .is_some_and(|a| matches!(a, "qwen35" | "bonsai2"))
                || self.model_name == "ternary-bonsai-2-27b-pq2"
            {
                4096
            } else {
                8192
            },
            max_output_tokens: Some(4096),
            supports_vision: false,
            supports_tools: false,             // TODO: implement tool calling
            supports_streaming: false,         // TODO: implement streaming inference
            input_cost_per_million: Some(0.0), // Free - local inference
            output_cost_per_million: Some(0.0),
        }])
    }
}
