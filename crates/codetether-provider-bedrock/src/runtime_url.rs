//! Bedrock Runtime model-endpoint URL construction.

use crate::BedrockClient;

impl BedrockClient<'_> {
    /// Build a runtime endpoint URL with the model ID encoded as one path segment.
    pub fn runtime_model_url(&self, model_id: &str, operation: &str) -> String {
        let model_id = urlencoding::encode(model_id);
        format!("{}/model/{model_id}/{operation}", self.base_url())
    }
}
