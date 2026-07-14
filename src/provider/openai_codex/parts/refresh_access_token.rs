impl OpenAiCodexProvider {
    async fn refresh_access_token(&self, refresh_token: &str) -> Result<OAuthCredentials> {
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

        #[derive(Deserialize)]
        struct TokenResponse {
            access_token: String,
            refresh_token: String,
            expires_in: u64,
        }

        let tokens: TokenResponse = response
            .json()
            .await
            .context("Failed to parse refresh response")?;

        let expires_at = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .context("System time error")?
            .as_secs()
            + tokens.expires_in;

        Ok(OAuthCredentials {
            id_token: None,
            chatgpt_account_id: self.chatgpt_account_id.clone(),
            access_token: tokens.access_token,
            refresh_token: tokens.refresh_token,
            expires_at,
        })
    }
}
