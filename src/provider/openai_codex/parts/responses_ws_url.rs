impl OpenAiCodexProvider {
    /// Returns the official OpenAI Responses WebSocket endpoint.
    ///
    /// # Examples
    ///
    /// ```
    /// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
    /// assert_eq!(
    ///     OpenAiCodexProvider::responses_ws_url(),
    ///     "wss://api.openai.com/v1/responses"
    /// );
    /// ```
    pub fn responses_ws_url() -> String {
        OPENAI_RESPONSES_WS_URL.to_string()
    }
}
