//! Delegate encoded runtime URLs to the client crate.
use crate::provider::bedrock::BedrockProvider;
impl BedrockProvider {
    /// Build a runtime endpoint URL with the model ID encoded as one path segment.
    pub(in crate::provider::bedrock) fn runtime_model_url(
        &self,
        model_id: &str,
        operation: &str,
    ) -> String {
        self.bedrock_client().runtime_model_url(model_id, operation)
    }
}
