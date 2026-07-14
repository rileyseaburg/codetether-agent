impl OpenAiCodexProvider {
    /// Loads local credentials for the explicitly opted-in ChatGPT backend.
    ///
    /// # Returns
    ///
    /// Credentials when backend opt-in and a complete Codex auth file exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let auth = OpenAiCodexProvider::local_chatgpt_backend_auth();
    /// assert!(auth.is_none() || auth.is_some());
    /// ```
    pub fn local_chatgpt_backend_auth() -> Option<ChatGptBackendAuth> {
        if !Self::chatgpt_backend_opted_in() {
            return None;
        }
        let credentials = Self::codex_auth_file_credentials()?;
        let account_id = credentials
            .chatgpt_account_id
            .or_else(|| Self::extract_chatgpt_account_id_from_jwt(&credentials.access_token))?;
        Some(ChatGptBackendAuth {
            access_token: credentials.access_token,
            account_id,
        })
    }
}
