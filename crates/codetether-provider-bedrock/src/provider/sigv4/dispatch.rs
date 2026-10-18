//! Compatibility delegation to the borrowed Bedrock HTTP client.
use super::BedrockProvider;
use anyhow::Result;
impl BedrockProvider {
    /// Send a pre-formed POST request to any Bedrock runtime URL, signed with
    /// whichever auth mode is configured. Used by the thinker backend for
    /// direct URL access.
    ///
    /// # Arguments
    ///
    /// * `url` — The full HTTPS URL to POST to.
    /// * `body` — The serialized JSON request body.
    ///
    /// # Errors
    ///
    /// Returns [`anyhow::Error`] if the network call fails.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_provider_bedrock::provider::{AwsCredentials, BedrockProvider};
    /// let creds = AwsCredentials::from_environment().unwrap();
    /// let p = BedrockProvider::with_credentials(creds, "us-east-1".into()).unwrap();
    /// let url = "https://bedrock-runtime.us-east-1.amazonaws.com/model/amazon.nova-micro-v1:0/converse";
    /// let resp = p.send_converse_request(url, b"{}" ).await.unwrap();
    /// assert!(resp.status().is_client_error() || resp.status().is_success());
    /// # });
    /// ```
    pub async fn send_converse_request(&self, url: &str, body: &[u8]) -> Result<reqwest::Response> {
        self.bedrock_client().send_converse_request(url, body).await
    }

    /// Send an HTTP request using whichever auth mode is configured.
    #[cfg(test)]
    pub(in crate::provider) async fn send_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&[u8]>,
        service: &str,
    ) -> Result<reqwest::Response> {
        self.bedrock_client()
            .send_request(method, url, body, service)
            .await
    }
}
