//! refresh access token.
use crate::*;
impl CodexOAuth {
    /// Exchange a refresh token using the supplied client and account identity.
    ///
    /// # Arguments
    /// * `client` - HTTP client configured by the provider.
    /// * `chatgpt_account_id` - Account identity retained on refreshed credentials.
    /// * `refresh_token` - Token to exchange.
    ///
    /// # Returns
    /// Updated credentials, retaining the sent refresh token when none is returned.
    ///
    /// # Errors
    /// Returns transport, response parsing, service rejection, or clock errors.
    ///
    /// # Examples
    /// ```no_run
    /// # async fn example(client: &reqwest::Client) -> anyhow::Result<()> {
    /// let credentials = codetether_codex_auth::CodexOAuth::request_refreshed_credentials(client, &None, "refresh-token").await?;
    /// assert!(!credentials.access_token.is_empty());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn request_refreshed_credentials(
        client: &reqwest::Client,
        chatgpt_account_id: &Option<String>,
        refresh_token: &str,
    ) -> Result<OAuthCredentials> {
        let form_body = format!(
            "grant_type={}&refresh_token={}&client_id={}",
            urlencoding::encode("refresh_token"),
            urlencoding::encode(refresh_token),
            CLIENT_ID,
        );

        let response = client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to refresh access token")?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("Token refresh failed: {}", body);
        }

        let tokens: RefreshTokenResponse = response
            .json()
            .await
            .context("Failed to parse refresh response")?;

        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + tokens.expires_in;

        Ok(OAuthCredentials {
            id_token: tokens.id_token,
            chatgpt_account_id: chatgpt_account_id.clone(),
            access_token: tokens.access_token,
            refresh_token: tokens
                .refresh_token
                .unwrap_or_else(|| refresh_token.to_string()),
            expires_at,
        })
    }
}
