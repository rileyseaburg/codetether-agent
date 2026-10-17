//! Sign and send one IAM-authenticated Bedrock request.
use super::*;
impl SigningClient<'_> {
    /// Build a SigV4-signed request and send it with the borrowed HTTP client.
    ///
    /// # Arguments
    /// * `method` - HTTP verb.
    /// * `url` - Full request URL, including any query parameters.
    /// * `body` - Bytes signed as the payload.
    /// * `service` - AWS service name used in the credential scope.
    ///
    /// # Returns
    /// The HTTP response, including unsuccessful status codes.
    ///
    /// # Errors
    /// Returns an error for bearer authentication, invalid URLs, or HTTP failure.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example(signer: codetether_bedrock_auth::sigv4::SigningClient<'_>) -> anyhow::Result<()> {
    /// let response = signer.send_signed_request("POST", "https://bedrock.us-east-1.amazonaws.com/", b"{}", "bedrock").await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn send_signed_request(
        &self,
        method: &str,
        url: &str,
        body: &[u8],
        service: &str,
    ) -> Result<reqwest::Response> {
        let creds = self.credentials()?;

        let now = chrono::Utc::now();
        let datestamp = now.format("%Y%m%d").to_string();
        let amz_date = now.format("%Y%m%dT%H%M%SZ").to_string();
        let canonical_url = canonicalize_url(url)?;
        let host = canonical_url.host;
        let canonical_uri = canonical_url.canonical_uri;
        let canonical_querystring = canonical_url.canonical_querystring;

        let payload_hash = sha256_hex(body);

        let (canonical_request, signed_headers) = super::headers::canonical_request(
            method,
            &canonical_uri,
            &canonical_querystring,
            &payload_hash,
            &host,
            &amz_date,
            creds,
        );

        let authorization = self.authorization(
            creds,
            &datestamp,
            service,
            &amz_date,
            &canonical_request,
            &signed_headers,
        );

        self.dispatch(
            method,
            url,
            body,
            creds,
            SignedHeaders {
                host,
                amz_date,
                payload_hash,
                authorization,
            },
        )
        .await
    }
}
