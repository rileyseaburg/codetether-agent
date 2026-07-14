impl OpenAiCodexProvider {
    /// Exchanges an OpenID identity token for an OpenAI API access token.
    ///
    /// # Arguments
    ///
    /// * `id_token` — Identity token issued by the OpenAI OAuth service.
    ///
    /// # Errors
    ///
    /// Returns an error when the request fails, the exchange is rejected, or
    /// the response contains no usable access token.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let access_token = OpenAiCodexProvider::exchange_id_token_for_api_key("id-token").await?;
    /// assert!(!access_token.is_empty());
    /// # Ok::<(), anyhow::Error>(())
    /// # }).unwrap();
    /// ```
    pub async fn exchange_id_token_for_api_key(id_token: &str) -> Result<String> {
        #[derive(Deserialize)]
        struct ExchangeResponse {
            access_token: String,
        }

        let client = crate::provider::shared_http::shared_client().clone();
        let form_body = format!(
            "grant_type={}&client_id={}&requested_token={}&subject_token={}&subject_token_type={}",
            urlencoding::encode("urn:ietf:params:oauth:grant-type:token-exchange"),
            urlencoding::encode(CLIENT_ID),
            urlencoding::encode("openai-api-key"),
            urlencoding::encode(id_token),
            urlencoding::encode("urn:ietf:params:oauth:token-type:id_token"),
        );

        let response = client
            .post(TOKEN_URL)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(form_body)
            .send()
            .await
            .context("Failed to exchange id_token for OpenAI API key")?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("API key token exchange failed ({}): {}", status, body);
        }

        let payload: ExchangeResponse = response
            .json()
            .await
            .context("Failed to parse API key token exchange response")?;
        if payload.access_token.trim().is_empty() {
            anyhow::bail!("API key token exchange returned an empty access token");
        }
        Ok(payload.access_token)
    }
}
