//! Bedrock model discovery delegated to the authenticated client crate.
use super::BedrockProvider;
use anyhow::Result;
use codetether_provider_core::ModelInfo;
impl BedrockProvider {
    /// Dynamically discover available Bedrock models.
    ///
    /// Queries both foundation models and system-defined inference profiles,
    /// filters to text-output chat models, and emits [`ModelInfo`] records.
    ///
    /// # Errors
    ///
    /// Returns an error only on unrecoverable network setup issues; individual
    /// API failures are logged and skipped, and the returned list may be empty.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::bedrock::{AwsCredentials, BedrockProvider};
    /// use codetether_agent::provider::Provider;
    ///
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-east-1".into()).unwrap();
    /// let models = p.list_models().await.unwrap();
    /// assert!(!models.is_empty());
    /// # });
    /// ```
    pub(super) async fn discover_models(&self) -> Result<Vec<ModelInfo>> {
        self.bedrock_client().discover_models().await
    }
}
