impl OpenAiCodexProvider {
    async fn connect_responses_ws_with_token(
        &self,
        token: &str,
        chatgpt_account_id: Option<&str>,
        backend: ResponsesWsBackend,
        session_id: &str,
    ) -> Result<OpenAiRealtimeConnection> {
        let mut request = match backend {
            ResponsesWsBackend::OpenAi => {
                Self::build_responses_ws_request_with_account_id(token, chatgpt_account_id)?
            }
            ResponsesWsBackend::ChatGptCodex => {
                Self::build_chatgpt_responses_ws_request(token, chatgpt_account_id)?
            }
        };
        ws_stream::turn_state::replay(&self.turn_states, session_id, &mut request);
        let connected = tokio::time::timeout(
            std::time::Duration::from_secs(15),
            connect_async(request),
        )
        .await
        .context("OpenAI Responses WebSocket connection timed out")?;
        let (stream, response) = connected
            .context("Failed to connect to OpenAI Responses WebSocket")?;
        if let Some(value) = response.headers().get(X_CODEX_TURN_STATE_HEADER)
            && let Ok(value) = value.to_str()
        {
            self.turn_states.capture(session_id, value);
        }
        Ok(OpenAiRealtimeConnection::new(stream))
    }
}
