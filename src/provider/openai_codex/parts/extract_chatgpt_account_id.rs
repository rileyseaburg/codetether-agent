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
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// assert_eq!(OpenAiCodexProvider::extract_chatgpt_account_id("not-a-jwt"), None);
    /// ```
    pub fn extract_chatgpt_account_id(token: &str) -> Option<String> {
        Self::extract_chatgpt_account_id_from_jwt(token)
    }
}
