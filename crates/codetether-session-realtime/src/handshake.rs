//! Validation of the first prompt frame on a realtime connection.

use super::frames::ClientFrame;
use super::socket::SocketStream;
use super::wire;

/// Receive the required non-empty prompt that begins one turn.
/// # Arguments
/// * `stream` — Receiving WebSocket half.
/// # Returns
/// Nonblank initial message and optional model.
/// # Errors
/// Returns an error for closure, invalid frames or a non-prompt first command.
/// # Examples
/// ```no_run
/// # async fn example(stream: &mut codetether_session_realtime::socket::SocketStream) {
/// let _ = codetether_session_realtime::handshake::prompt(stream).await;
/// # }
/// ```
pub async fn prompt(stream: &mut SocketStream) -> Result<(String, Option<String>), String> {
    let Some(frame) = wire::next(stream).await? else {
        return Err("Connection closed before a prompt was sent.".into());
    };
    match frame {
        ClientFrame::Prompt { message, model } if !message.trim().is_empty() => {
            Ok((message, model))
        }
        ClientFrame::Prompt { .. } => Err("Prompt text is empty.".into()),
        ClientFrame::Steer { .. } | ClientFrame::Cancel => {
            Err("The first realtime frame must be a prompt.".into())
        }
    }
}
