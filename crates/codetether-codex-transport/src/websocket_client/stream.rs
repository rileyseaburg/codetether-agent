//! Replay routing state, obtain a connection, and start one response stream.
use super::*;
impl WebSocketTransport<'_> {
    /// Replay routing state, acquire a connection, and start the response driver.
    ///
    /// # Errors
    /// Returns connection setup errors before streaming begins.
    pub async fn start_stream(
        &self,
        mut body: Value,
        access_token: String,
        chatgpt_account_id: Option<String>,
        backend: &'static str,
        ws_backend: ResponsesWsBackend,
        session_id: &str,
    ) -> Result<Chunks> {
        if let Some(value) = self.turn_states.current(session_id).get() {
            body["client_metadata"] = json!({ (X_CODEX_TURN_STATE_HEADER): value });
        }
        crate::request_log::log_responses_ws_request(&body, backend);

        let connection = self
            .pooled_ws_connection(
                &access_token,
                chatgpt_account_id.as_deref(),
                ws_backend,
                session_id,
            )
            .await?;

        Ok(websocket::drive(
            connection,
            body,
            session_id.to_string(),
            self.transport_health.clone(),
            self.turn_states.clone(),
            self.ws_pool.clone(),
        ))
    }
}
