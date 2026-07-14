/// Typed WebSocket connection for OpenAI Responses events.
///
/// The connection serializes and parses each event as JSON while handling
/// WebSocket control frames internally.
///
/// # Examples
///
/// ```rust,no_run
/// # tokio::runtime::Runtime::new().unwrap().block_on(async {
/// use codetether_agent::provider::openai_codex::OpenAiCodexProvider;
/// let provider = OpenAiCodexProvider::from_api_key("api-key".to_string());
/// let _connection = provider.connect_responses_ws().await?;
/// # Ok::<(), anyhow::Error>(())
/// # }).unwrap();
/// ```
pub struct OpenAiRealtimeConnection<S = MaybeTlsStream<TcpStream>> {
    stream: WebSocketStream<S>,
}

impl<S> std::fmt::Debug for OpenAiRealtimeConnection<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OpenAiRealtimeConnection").finish()
    }
}

impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    fn new(stream: WebSocketStream<S>) -> Self {
        Self { stream }
    }

    /// Sends one JSON event to the Responses WebSocket.
    ///
    /// # Errors
    ///
    /// Returns an error when JSON serialization or WebSocket transmission fails.
    pub async fn send_event(&mut self, event: &Value) -> Result<()> {
        let payload = serde_json::to_string(event).context("Failed to serialize Realtime event")?;
        self.stream
            .send(WsMessage::Text(payload.into()))
            .await
            .context("Failed to send Realtime event")?;
        Ok(())
    }
}
