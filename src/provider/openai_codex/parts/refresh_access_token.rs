impl OpenAiCodexProvider {
    async fn request_refreshed_credentials(
        &self,
        refresh_token: &str,
    ) -> Result<OAuthCredentials> {
        let form_body = format!(
            "grant_type={}&refresh_token={}&client_id={}",
            urlencoding::encode("refresh_token"),
            urlencoding::encode(refresh_token),
            CLIENT_ID,
        );

        let response = self
            .client
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
            chatgpt_account_id: self.chatgpt_account_id.clone(),
            access_token: tokens.access_token,
            refresh_token: tokens
                .refresh_token
                .unwrap_or_else(|| refresh_token.to_string()),
            expires_at,
        })
    }
}
