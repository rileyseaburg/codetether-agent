impl OpenAiCodexProvider {
    async fn send_chatgpt_http_response(
        &self,
        request: &CompletionRequest,
        access_token: &str,
        account_id: &str,
        session_id: &str,
    ) -> Result<reqwest::Response> {
        let body = Self::build_http_responses_body(request);
        Self::log_http_responses_request("chatgpt-codex-responses-http", &body);
        codetether_codex_transport::HttpTransport::new(&self.client, &self.turn_states)
            .send_chatgpt_http_response(&body, access_token, account_id, session_id)
            .await
    }
}
