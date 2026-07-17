impl OpenAiCodexProvider {
    /// Creates a provider from persisted OAuth credentials.
    ///
    /// Credentials from the Codex CLI auth file take precedence when present.
    ///
    /// # Arguments
    ///
    /// * `credentials` — Persisted access, refresh, identity, and workspace data.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::{OAuthCredentials, OpenAiCodexProvider};
    /// let credentials = OAuthCredentials {
    ///     id_token: None,
    ///     chatgpt_account_id: None,
    ///     access_token: "access".to_string(),
    ///     refresh_token: "refresh".to_string(),
    ///     expires_at: u64::MAX,
    /// };
    /// let provider = OpenAiCodexProvider::from_credentials(credentials);
    /// assert!(format!("{provider:?}").contains("has_credentials: true"));
    /// ```
    pub fn from_credentials(credentials: OAuthCredentials) -> Self {
        let credentials = Self::codex_auth_file_credentials().unwrap_or(credentials);
        let chatgpt_account_id = credentials
            .chatgpt_account_id
            .clone()
            .or_else(|| {
                credentials
                    .id_token
                    .as_deref()
                    .and_then(Self::extract_chatgpt_account_id_from_jwt)
            })
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(&credentials.access_token));

        Self {
            client: crate::provider::shared_http::shared_client().clone(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: None,
            chatgpt_account_id,
            stored_credentials: Some(credentials),
            transport_health: TransportHealth::default(),
            ws_pool: WsPool::default(),
        }
    }
}
