//! Serialization of server frames onto the WebSocket sink.

use axum::extract::ws::Message;
use futures::SinkExt;

use super::frames::ServerFrame;
use super::socket::SocketSink;

/// Serialize and send one ordered server frame.
pub(super) async fn frame(sink: &mut SocketSink, frame: &ServerFrame) -> Result<(), String> {
    let encoded = serde_json::to_string(frame).map_err(error_text)?;
    sink.send(Message::Text(encoded.into()))
        .await
        .map_err(error_text)
}

/// Convert a displayable transport failure into its wire-safe text.
fn error_text(error: impl std::fmt::Display) -> String {
    error.to_string()
}
