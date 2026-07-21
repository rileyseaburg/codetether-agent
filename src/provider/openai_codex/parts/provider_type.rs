#[derive(Clone)]
/// Completion provider for the OpenAI Responses API and Codex OAuth backend.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
///
/// let provider = OpenAiCodexProvider::new();
/// assert!(format!("{provider:?}").contains("OpenAiCodexProvider"));
/// ```
pub struct OpenAiCodexProvider {
    client: Client,
    cached_tokens: Arc<RwLock<Option<CachedTokens>>>,
    static_api_key: Option<String>,
    chatgpt_account_id: Option<String>,
    stored_credentials: Option<Arc<RwLock<OAuthCredentialState>>>,
    credential_store: Option<VaultCredentialStore>,
    transport_health: TransportHealth,
    turn_states: TurnStateStore,
    ws_pool: WsPool,
}

impl std::fmt::Debug for OpenAiCodexProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiCodexProvider")
            .field("has_api_key", &self.static_api_key.is_some())
            .field("has_chatgpt_account_id", &self.chatgpt_account_id.is_some())
            .field("has_credentials", &self.stored_credentials.is_some())
            .finish()
    }
}

impl Default for OpenAiCodexProvider {
    fn default() -> Self {
        Self::new()
    }
}
