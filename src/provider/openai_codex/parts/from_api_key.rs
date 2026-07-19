impl OpenAiCodexProvider {
    /// Creates a provider authenticated with an OpenAI API key.
    ///
    /// # Arguments
    ///
    /// * `api_key` — Bearer credential for the official OpenAI API.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let provider = OpenAiCodexProvider::from_api_key("test-key".to_string());
    /// assert!(format!("{provider:?}").contains("has_api_key: true"));
    /// ```
    pub fn from_api_key(api_key: String) -> Self {
        Self {
            client: crate::provider::shared_http::shared_client().clone(),
            cached_tokens: Arc::new(RwLock::new(None)),
            static_api_key: Some(api_key),
            chatgpt_account_id: None,
            stored_credentials: None,
            transport_health: TransportHealth::default(),
            turn_states: TurnStateStore::default(),
            ws_pool: WsPool::default(),
        }
    }
}
