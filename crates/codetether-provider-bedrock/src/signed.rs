//! Borrow provider state for IAM request signing.
use super::BedrockClient;
use anyhow::Result;
impl BedrockClient<'_> {
    pub(crate) async fn send_signed_request(
        &self,
        method: &str,
        url: &str,
        body: &[u8],
        service: &str,
    ) -> Result<reqwest::Response> {
        codetether_bedrock_auth::sigv4::SigningClient::new(&self.client, &self.auth, &self.region)
            .send_signed_request(method, url, body, service)
            .await
    }
}
