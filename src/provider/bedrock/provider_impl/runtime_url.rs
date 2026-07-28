//! Bedrock Runtime model-endpoint URL construction.

use crate::provider::bedrock::BedrockProvider;

impl BedrockProvider {
    /// Build a runtime endpoint URL with the model ID encoded as one path segment.
    pub(in crate::provider::bedrock) fn runtime_model_url(
        &self,
        model_id: &str,
        operation: &str,
    ) -> String {
        let model_id = urlencoding::encode(model_id);
        format!("{}/model/{model_id}/{operation}", self.base_url())
    }
}
