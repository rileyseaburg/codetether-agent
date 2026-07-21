impl OpenAiCodexProvider {
    /// Creates a provider from persisted OAuth credentials.
    ///
    /// The supplied credentials are used directly. Callers are responsible for
    /// loading them from the configured secret backend.
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
            stored_credentials: Some(Arc::new(RwLock::new(OAuthCredentialState::new(credentials)))),
            credential_store: None,
            transport_health: TransportHealth::default(),
            turn_states: TurnStateStore::default(),
            ws_pool: WsPool::default(),
        }
    }

    /// Creates an OAuth provider whose refreshed credentials persist to Vault.
    pub(crate) fn from_vault_credentials(
        provider_id: &str,
        credentials: OAuthCredentials,
    ) -> Self {
        let mut provider = Self::from_credentials(credentials);
        provider.credential_store = Some(VaultCredentialStore::new(provider_id));
        provider
    }
}
