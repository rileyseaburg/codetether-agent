impl OpenAiCodexProvider {
    async fn connect_responses_ws_with_token(
        &self,
        token: &str,
        chatgpt_account_id: Option<&str>,
        backend: ResponsesWsBackend,
    ) -> Result<OpenAiRealtimeConnection> {
        let request = match backend {
            ResponsesWsBackend::OpenAi => {
                Self::build_responses_ws_request_with_account_id(token, chatgpt_account_id)?
            }
            ResponsesWsBackend::ChatGptCodex => {
                Self::build_chatgpt_responses_ws_request(token, chatgpt_account_id)?
            }
        };
        let (stream, _response) = connect_async(request)
            .await
            .context("Failed to connect to OpenAI Responses WebSocket")?;
        Ok(OpenAiRealtimeConnection::new(stream))
    }
}
