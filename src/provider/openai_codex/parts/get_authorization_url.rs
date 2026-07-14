impl OpenAiCodexProvider {
    /// Builds an OAuth authorization URL and its PKCE verifier and state value.
    ///
    /// # Returns
    ///
    /// Returns `(authorization_url, pkce_verifier, state)`.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// let (url, verifier, state) = OpenAiCodexProvider::get_authorization_url();
    /// assert!(url.starts_with("https://auth.openai.com/oauth/authorize"));
    /// assert!(!verifier.is_empty() && !state.is_empty());
    /// ```
    pub fn get_authorization_url() -> (String, String, String) {
        let pkce = Self::generate_pkce();
        let state = Self::generate_state();

        let url = format!(
            "{}?response_type=code&client_id={}&redirect_uri={}&scope={}&code_challenge={}&code_challenge_method=S256&state={}&id_token_add_organizations=true&codex_cli_simplified_flow=true&originator=codex_cli_rs",
            AUTHORIZE_URL,
            CLIENT_ID,
            urlencoding::encode(REDIRECT_URI),
            urlencoding::encode(SCOPE),
            pkce.challenge,
            state
        );

        (url, pkce.verifier, state)
    }
}
