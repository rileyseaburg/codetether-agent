//! Credential validation and configured HTTP client construction.
use crate::*;
impl VertexGlmProvider {
    /// Create a provider from a service-account JSON key.
    ///
    /// # Arguments
    /// * `sa_json` - JSON service-account credentials.
    /// * `project_id` - Optional project override.
    ///
    /// # Returns
    /// A provider with a cached token service and configured HTTP client.
    ///
    /// # Errors
    /// Returns an error for invalid credentials, project, or client settings.
    ///
    /// # Examples
    /// See the construction example on [VertexGlmProvider].
    pub fn new(sa_json: &str, project_id: Option<String>) -> Result<Self> {
        let (auth, project_id) =
            codetether_google_auth::ServiceAccountAuth::from_json(sa_json, project_id)?;

        let base_url = format!(
            "https://{}/v1/projects/{}/locations/{}/endpoints/openapi",
            VERTEX_ENDPOINT, project_id, VERTEX_REGION
        );

        tracing::debug!(
            provider = "vertex-glm",
            project_id = %project_id,
            client_email = %auth.client_email(),
            base_url = %base_url,
            "Creating Vertex GLM provider with service account"
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
