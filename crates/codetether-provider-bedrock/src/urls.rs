//! Runtime and management endpoint URLs.
use super::BedrockClient;
impl BedrockClient<'_> {
    /// Runtime Bedrock base URL (Converse / Converse-stream).
    ///
    /// # Examples
    /// See [BedrockClient] for construction.
    ///
    /// # Arguments
    /// Uses the configured region.
    ///
    /// # Returns
    /// The Bedrock runtime base URL.
    pub fn base_url(&self) -> String {
        format!("https://bedrock-runtime.{}.amazonaws.com", self.region)
    }
    /// Management API URL (ListFoundationModels, ListInferenceProfiles).
    ///
    /// # Arguments
    /// Uses the configured region.
    ///
    /// # Returns
    /// The Bedrock management base URL.
    ///
    /// # Examples
    /// See [BedrockClient] for construction.
    pub fn management_url(&self) -> String {
        format!("https://bedrock.{}.amazonaws.com", self.region)
    }
}
