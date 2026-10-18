//! Compatibility delegation to the borrowed Bedrock HTTP client.
use super::BedrockProvider;
impl BedrockProvider {
    /// Runtime Bedrock base URL (Converse / Converse-stream).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use codetether_provider_bedrock::provider::BedrockProvider;
    /// let p = BedrockProvider::with_region("token".into(), "us-east-1".into()).unwrap();
    /// // internal: p.base_url() -> "https://bedrock-runtime.us-east-1.amazonaws.com"
    /// ```
    #[cfg(test)]
    pub(in crate::provider) fn base_url(&self) -> String {
        self.bedrock_client().base_url()
    }

    /// Management API URL (ListFoundationModels, ListInferenceProfiles).
    #[cfg(test)]
    pub(in crate::provider) fn management_url(&self) -> String {
        self.bedrock_client().management_url()
    }
}
