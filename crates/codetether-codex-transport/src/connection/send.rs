//! Connection send_event.
use super::*;
use anyhow::{Context, Result};
use futures::SinkExt;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message as WsMessage;
impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
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
