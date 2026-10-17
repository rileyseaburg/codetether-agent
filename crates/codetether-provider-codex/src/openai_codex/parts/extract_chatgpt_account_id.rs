impl OpenAiCodexProvider {
    /// Extracts a ChatGPT workspace identifier from a JWT claim set.
    ///
    /// # Arguments
    ///
    /// * `token` — JWT access or identity token.
    ///
    /// # Returns
    ///
    /// Returns the workspace identifier when a recognized claim is present.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_provider_codex::OpenAiCodexProvider;
    /// assert_eq!(OpenAiCodexProvider::extract_chatgpt_account_id("not-a-jwt"), None);
    /// ```
    pub fn extract_chatgpt_account_id(token: &str) -> Option<String> {
        codetether_codex_auth::CodexOAuth::extract_chatgpt_account_id(token)
    }
}
