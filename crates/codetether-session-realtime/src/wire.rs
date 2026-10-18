//! Parsing of bounded client WebSocket messages.

use axum::extract::ws::Message;
use futures::StreamExt;

use super::frames::ClientFrame;
use super::socket::SocketStream;

/// Read the next command while ignoring transport-level ping frames.
/// # Arguments
/// * `stream` — Receiving WebSocket half.
/// # Returns
/// Next decoded command or None on closure.
/// # Errors
/// Returns text for transport, JSON or binary-frame errors.
/// # Examples
/// ```no_run
/// # async fn example(stream: &mut codetether_session_realtime::socket::SocketStream) {
/// let _ = codetether_session_realtime::wire::next(stream).await;
/// # }
/// ```
pub async fn next(stream: &mut SocketStream) -> Result<Option<ClientFrame>, String> {
    loop {
        let Some(message) = stream.next().await else {
            return Ok(None);
        };
        match message.map_err(|error| error.to_string())? {
            Message::Text(text) => return decode(text.as_str()).map(Some),
            Message::Close(_) => return Ok(None),
            Message::Ping(_) | Message::Pong(_) => continue,
            Message::Binary(_) => {
                return Err("Binary realtime frames are unsupported.".into());
            }
        }
    }
}

/// Decode one text command into the strict client frame contract.
/// # Arguments
/// * `text` — JSON command.
/// # Returns
/// Decoded client command.
/// # Errors
/// Returns the JSON decoding error for malformed input.
/// # Examples
/// ```no_run
/// # fn example(text: &str) {
/// let _ = codetether_session_realtime::wire::decode(text);
/// # }
/// ```
pub fn decode(text: &str) -> Result<ClientFrame, String> {
    serde_json::from_str(text).map_err(|error| error.to_string())
}
