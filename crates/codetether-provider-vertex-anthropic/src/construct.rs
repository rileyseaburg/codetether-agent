//! Provider-specific endpoint and HTTP client construction.
use crate::*;
impl VertexAnthropicProvider {
    /// Create from a service account JSON key string
    ///
    /// # Arguments
    /// * sa_json - Service-account credentials as JSON.
    /// * project_id - Optional project override.
    ///
    /// # Returns
    /// A provider with its original Vertex endpoint and HTTP timeout policy.
    ///
    /// # Errors
    /// Returns credential parsing, project selection, RSA key, or client errors.
    ///
    /// # Examples
    ///
    ///     assert!(codetether_provider_vertex_anthropic::VertexAnthropicProvider::new("{}", None).is_err());
    pub fn new(sa_json: &str, project_id: Option<String>) -> Result<Self> {
        let (auth, project_id) =
            codetether_google_auth::ServiceAccountAuth::from_json(sa_json, project_id)?;

        let base_url = format!(
            "https://{}-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/anthropic/models",
            VERTEX_REGION, project_id, VERTEX_REGION
        );

        tracing::debug!(
            provider = "vertex-anthropic",
            project_id = %project_id,
            client_email = %auth.client_email(),
            base_url = %base_url,
            "Creating Vertex Anthropic provider with service account"
        );

        let client = codetether_provider_core::socket_opts::apply_socket_opts(Client::builder())
            .connect_timeout(CONNECT_TIMEOUT)
            .timeout(REQUEST_TIMEOUT)
            .build()
            .context("Failed to build HTTP client")?;

        Ok(Self {
            client,
            project_id,
            base_url,
            auth,
        })
    }
    pub(crate) async fn get_access_token(&self) -> Result<String> {
        self.auth.access_token(&self.client).await
    }
}
