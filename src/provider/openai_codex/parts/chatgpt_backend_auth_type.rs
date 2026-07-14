/// Credentials required by the opted-in ChatGPT Codex backend.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::provider::openai_codex::ChatGptBackendAuth;
/// let auth = ChatGptBackendAuth {
///     access_token: "token".into(),
///     account_id: "account".into(),
/// };
/// assert_eq!(auth.account_id, "account");
/// ```
#[derive(Clone)]
pub struct ChatGptBackendAuth {
    /// OAuth bearer token used by the ChatGPT backend.
    pub access_token: String,
    /// ChatGPT workspace account sent in the account header.
    pub account_id: String,
}
