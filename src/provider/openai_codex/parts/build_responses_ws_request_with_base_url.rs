impl OpenAiCodexProvider {
    /// Builds a test-only WebSocket request for the OpenAI Responses API using
    /// the provided base URL and bearer token.
    ///
    /// This is a convenience wrapper around
    /// [`Self::build_responses_ws_request_with_base_url_and_account_id`] that
    /// does not include an account identifier in the generated request.
    ///
    /// # Parameters
    ///
    /// - `base_url`: Base endpoint used to construct the WebSocket request URL.
    /// - `token`: Authentication token to include in the request.
    ///
    /// # Returns
    ///
    /// Returns a [`Request<()>`] configured for the Responses WebSocket endpoint,
    /// or an error if the request cannot be constructed from the supplied input.
    ///
    /// # Availability
    ///
    /// This helper is compiled only for tests.
    #[cfg(test)]
    fn build_responses_ws_request_with_base_url(
        base_url: &str,
        token: &str,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(base_url, token, None)
    }
}