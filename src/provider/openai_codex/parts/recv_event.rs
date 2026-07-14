impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    /// Receives the next JSON event, responding to ping frames as needed.
    ///
    /// # Returns
    ///
    /// Returns `None` after the peer closes the connection or the stream ends.
    ///
    /// # Errors
    ///
    /// Returns an error for transport failures, invalid UTF-8, or invalid JSON.
    pub async fn recv_event(&mut self) -> Result<Option<Value>> {
        while let Some(message) = self.stream.next().await {
            let message = message.context("Realtime WebSocket receive failed")?;
            match message {
                WsMessage::Text(text) => {
                    let event = serde_json::from_str(&text)
                        .context("Failed to parse Realtime text event")?;
                    return Ok(Some(event));
                }
                WsMessage::Binary(bytes) => {
                    let text = String::from_utf8(bytes.to_vec())
                        .context("Realtime binary event was not valid UTF-8")?;
                    let event = serde_json::from_str(&text)
                        .context("Failed to parse Realtime binary event")?;
                    return Ok(Some(event));
                }
                WsMessage::Ping(payload) => {
                    self.stream
                        .send(WsMessage::Pong(payload))
                        .await
                        .context("Failed to respond to Realtime ping")?;
                }
                WsMessage::Pong(_) => {}
                WsMessage::Frame(_) => {}
                WsMessage::Close(_) => return Ok(None),
            }
        }

        Ok(None)
    }
}
