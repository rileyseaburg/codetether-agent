//! Native construction boundaries requiring application state or identity.
use codetether_bedrock_auth::AwsCredentials;
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Construct providers whose HTTP pool, User-Agent or runtime belongs to the app.
///
/// Each method returns its constructor error unchanged; factory policy logs it
/// and skips the provider using the existing dispatch behavior.
pub trait Host {
    /// Create a Bedrock provider using application HTTP pooling and IAM credentials.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn bedrock_with_credentials(
        credentials: AwsCredentials,
        region: String,
    ) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a Bedrock provider using a bearer credential.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn bedrock_with_region(key: String, region: String) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a local CUDA provider with the application inference runtime.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn local_cuda_new(model: String) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a local CUDA provider with explicit model artifacts.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn local_cuda_with_paths(
        model: String,
        path: String,
        tokenizer: Option<String>,
        architecture: Option<String>,
    ) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a public Copilot provider with application identity.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn copilot_new(key: String) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a Copilot provider with an explicit API endpoint and identity.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn copilot_with_base_url(
        key: String,
        url: String,
        name: &str,
    ) -> anyhow::Result<Arc<dyn Provider>>;
    /// Create a Copilot provider for an enterprise endpoint.
    /// # Errors
    /// Returns the underlying native constructor failure.
    fn copilot_enterprise(key: String, url: String) -> anyhow::Result<Arc<dyn Provider>>;
}
