//! Send a signed request without changing the caller's HTTP client.
use super::*;
impl SigningClient<'_> {
    pub(super) async fn dispatch(
        &self,
        method: &str,
        url: &str,
        body: &[u8],
        creds: &AwsCredentials,
        signed: SignedHeaders,
    ) -> Result<reqwest::Response> {
        let mut req = self
            .client
            .request(method.parse().unwrap_or(reqwest::Method::POST), url)
            .header("content-type", "application/json")
            .header("host", &signed.host)
            .header("x-amz-date", &signed.amz_date)
            .header("x-amz-content-sha256", &signed.payload_hash)
            .header("authorization", &signed.authorization);

        if let Some(token) = &creds.session_token {
            req = req.header("x-amz-security-token", token);
        }

        if method == "POST" || method == "PUT" {
            req = req.body(body.to_vec());
        }

        req.send()
            .await
            .context("Failed to send signed request to Bedrock")
    }
}
