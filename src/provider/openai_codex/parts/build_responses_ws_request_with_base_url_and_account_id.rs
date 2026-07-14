// Construction utilities for OpenAI Codex Responses WebSocket requests.
//
// This included file contains request-building logic for the Responses WebSocket
// transport. It prepares the HTTP upgrade request with the authentication and
// optional ChatGPT account headers required by the upstream service.

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
        if token.trim().is_empty() {
            anyhow::bail!("Responses WebSocket token cannot be empty");
        }

        let url = base_url.to_string();
        let mut request = url
            .into_client_request()
            .context("Failed to build Responses WebSocket request")?;
        request.headers_mut().insert(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token}"))
                .context("Failed to build Responses Authorization header")?,
        );
        request.headers_mut().insert(
            "User-Agent",
            HeaderValue::from_static("codetether-responses-ws/1.0"),
        );
        if let Some(account_id) = chatgpt_account_id.filter(|id| !id.trim().is_empty()) {
            request.headers_mut().insert(
                "ChatGPT-Account-ID",
                HeaderValue::from_str(account_id)
                    .context("Failed to build ChatGPT account header")?,
            );
        }
        Ok(request)
    }
}
