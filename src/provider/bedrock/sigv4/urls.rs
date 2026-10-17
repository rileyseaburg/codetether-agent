//! Compatibility delegation to the borrowed Bedrock HTTP client.
use super::BedrockProvider;
impl BedrockProvider {
    /// Runtime Bedrock base URL (Converse / Converse-stream).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_agent::provider::bedrock::BedrockProvider;
    /// let p = BedrockProvider::with_region("token".into(), "us-east-1".into()).unwrap();
    /// // internal: p.base_url() -> "https://bedrock-runtime.us-east-1.amazonaws.com"
    /// ```
    pub(in crate::provider::bedrock) fn base_url(&self) -> String {
        self.bedrock_client().base_url()
    }

    /// Management API URL (ListFoundationModels, ListInferenceProfiles).
    pub(in crate::provider::bedrock) fn management_url(&self) -> String {
        self.bedrock_client().management_url()
    }
}
