impl OpenAiCodexProvider {
    fn build_chatgpt_responses_ws_request(
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(
            CHATGPT_CODEX_RESPONSES_WS_URL,
            token,
            chatgpt_account_id,
        )
    }
}
