impl OpenAiCodexProvider {
    fn build_responses_ws_request_with_account_id(
        token: &str,
        chatgpt_account_id: Option<&str>,
    ) -> Result<Request<()>> {
        Self::build_responses_ws_request_with_base_url_and_account_id(
            OPENAI_RESPONSES_WS_URL,
            token,
            chatgpt_account_id,
        )
    }
}
