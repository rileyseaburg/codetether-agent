#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResponsesWsBackend {
    OpenAi,
    ChatGptCodex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CodexServiceTier {
    Priority,
}

impl CodexServiceTier {
    fn as_str(self) -> &'static str {
        match self {
            Self::Priority => "priority",
        }
    }
}

/// Cached OAuth tokens with expiration tracking
struct CachedTokens {
    access_token: String,
    expires_at: std::time::Instant,
}

/// OAuth credentials persisted by the provider's secret store.
///
/// # Examples
///
/// ```
/// use codetether_agent::provider::openai_codex::OAuthCredentials;
///
/// let credentials = OAuthCredentials {
///     id_token: None,
///     chatgpt_account_id: Some("workspace-1".to_string()),
///     access_token: "access".to_string(),
///     refresh_token: "refresh".to_string(),
///     expires_at: 1_900_000_000,
/// };
/// assert_eq!(credentials.chatgpt_account_id.as_deref(), Some("workspace-1"));
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCredentials {
    /// Optional OpenID Connect identity token returned by OAuth.
    #[serde(default)]
    pub id_token: Option<String>,
    /// ChatGPT workspace identifier associated with the credentials.
    #[serde(default)]
    pub chatgpt_account_id: Option<String>,
    /// Bearer token used for authenticated requests.
    pub access_token: String,
    /// Token used to renew an expired access token.
    pub refresh_token: String,
    /// Access-token expiration as Unix seconds.
    pub expires_at: u64,
}

/// PKCE code verifier and challenge pair
struct PkcePair {
    verifier: String,
    challenge: String,
}

#[derive(Debug, Default)]
struct ResponsesToolState {
    call_id: String,
    name: Option<String>,
    started: bool,
    finished: bool,
    emitted_arguments: String,
}

#[derive(Debug, Default)]
struct ResponsesSseParser {
    line_buffer: String,
    event_data_lines: Vec<String>,
    tools: HashMap<String, ResponsesToolState>,
}
