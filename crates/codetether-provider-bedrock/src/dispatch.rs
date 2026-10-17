//! Authentication dispatch using the provider client and current bearer token.
use super::{BedrockAuth, BedrockClient};
use anyhow::{Context, Result};
impl BedrockClient<'_> {
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
    /// See [BedrockClient] for construction.
    ///
    /// # Returns
    /// The HTTP response for the signed or bearer-authenticated request.
    pub async fn send_converse_request(&self, url: &str, body: &[u8]) -> Result<reqwest::Response> {
        self.send_request("POST", url, Some(body), "bedrock").await
    }
    /// Send an HTTP request using whichever auth mode is configured.
    ///
    /// # Arguments
    /// * `method` - HTTP method string.
    /// * `url` - Full request URL.
    /// * `body` - Optional bytes for the request.
    /// * `service` - AWS service for IAM signing.
    ///
    /// # Returns
    /// The HTTP response, including error status codes.
    ///
    /// # Errors
    /// Returns an error if signing or sending the request fails.
    ///
    /// # Examples
    /// See [BedrockClient] for construction.
    pub async fn send_request(
        &self,
        method: &str,
        url: &str,
        body: Option<&[u8]>,
        service: &str,
    ) -> Result<reqwest::Response> {
        match &self.auth {
            BedrockAuth::SigV4(_) => {
                self.send_signed_request(method, url, body.unwrap_or(b""), service)
                    .await
            }
            BedrockAuth::BearerToken(cell) => {
                let mut req = self
                    .client
                    .request(method.parse().unwrap_or(reqwest::Method::GET), url)
                    .bearer_auth(cell.read().clone())
                    .header("content-type", "application/json")
                    .header("accept", "application/json");
                if let Some(b) = body {
                    req = req.body(b.to_vec());
                }
                req.send()
                    .await
                    .context("Failed to send request to Bedrock")
            }
        }
    }
}
