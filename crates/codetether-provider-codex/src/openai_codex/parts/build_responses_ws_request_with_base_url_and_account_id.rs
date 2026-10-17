impl OpenAiCodexProvider {
    /// Builds an HTTP request suitable for opening a Responses WebSocket connection.
    ///
    /// The returned request is created from `base_url` and populated with the
    /// headers required by the Responses WebSocket endpoint:
    ///
    /// - `Authorization: Bearer <token>`
    /// - `User-Agent: codetether-responses-ws/1.0`
    /// - `ChatGPT-Account-ID: <id>` when `chatgpt_account_id` is present and
    ///   not blank
    ///
    /// # Parameters
    ///
    /// * `base_url` - WebSocket endpoint URL used to create the client request.
    ///   It must be accepted by `IntoClientRequest`.
    /// * `token` - Bearer token used for authentication. Leading and trailing
    ///   whitespace is ignored only when checking whether the token is empty;
    ///   the original value is used in the header.
    /// * `chatgpt_account_id` - Optional ChatGPT account identifier to attach
    ///   to the request. Blank values are ignored.
    ///
    /// # Returns
    ///
    /// Returns a `Request<()>` containing the WebSocket request and configured
    /// headers.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    ///
    /// * `token` is empty or contains only whitespace.
    /// * `base_url` cannot be converted into a client request.
    /// * the bearer token or account ID cannot be represented as a valid HTTP
    ///   header value.
    fn build_responses_ws_request_with_base_url_and_account_id(
        base_url: &str,
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        codetether_codex_transport::websocket::build_responses_ws_request_with_base_url_and_account_id(base_url,token,chatgpt_account_id)
    }
}
