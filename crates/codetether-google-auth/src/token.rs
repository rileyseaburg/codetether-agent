//! Cached token lookup, refresh, and publication.
use super::*;
impl ServiceAccountAuth {
    /// Read the cache or sign and exchange a fresh access-token assertion.
    ///
    /// # Arguments
    /// * client - The provider's existing HTTP client and timeout policy.
    ///
    /// # Returns
    /// A cached or newly exchanged access token.
    ///
    /// # Errors
    /// Returns clock, signing, HTTP, non-success response, or JSON errors.
    ///
    /// # Examples
    ///
    ///     # async fn example(auth: &codetether_google_auth::ServiceAccountAuth, client: &reqwest::Client) -> anyhow::Result<()> {
    ///     let token = auth.access_token(client).await?;
    ///     let _ = token;
    ///     # Ok(()) }
    pub async fn access_token(&self, client: &Client) -> Result<String> {
        // Check cache — refresh 5 minutes before expiration
        {
            let cache = self.cached_token.read().await;
            if let Some(ref cached) = *cache
                && cached.expires_at
                    > std::time::Instant::now() + std::time::Duration::from_secs(300)
            {
                return Ok(cached.token.clone());
            }
        }

        let (assertion, token_uri) = self.assertion()?;
        let token_resp = self.exchange(client, token_uri, &assertion).await?;
        let expires_in = token_resp.expires_in.unwrap_or(3600);

        // Cache it
        {
            let mut cache = self.cached_token.write().await;
            *cache = Some(CachedToken {
                token: token_resp.access_token.clone(),
                expires_at: std::time::Instant::now() + std::time::Duration::from_secs(expires_in),
            });
        }

        tracing::debug!(
            client_email = %self.sa_key.client_email,
            expires_in_secs = expires_in,
            "Refreshed GCP access token via service account JWT"
        );

        Ok(token_resp.access_token)
    }
}
