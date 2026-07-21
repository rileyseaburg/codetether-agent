impl OpenAiCodexProvider {
    /// Creates an unauthenticated provider for use with a subsequent OAuth flow.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let provider = OpenAiCodexProvider::new();
    /// assert!(format!("{provider:?}").contains("has_api_key: false"));
    /// ```
    pub fn new() -> Self {
        Self {
            client: crate::provider::shared_http::shared_client().clone(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: None,
            chatgpt_account_id: None,
            stored_credentials: None,
            credential_store: None,
            transport_health: TransportHealth::default(),
            turn_states: TurnStateStore::default(),
            ws_pool: WsPool::default(),
        }
    }
}
