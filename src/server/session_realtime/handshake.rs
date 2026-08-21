//! Validation of the first prompt frame on a realtime connection.

use super::frames::ClientFrame;
use super::socket::SocketStream;
use super::wire;

/// Receive the required non-empty prompt that begins one turn.
pub(super) async fn prompt(stream: &mut SocketStream) -> Result<String, String> {
    let Some(frame) = wire::next(stream).await? else {
        return Err("Connection closed before a prompt was sent.".into());
    };
    match frame {
        ClientFrame::Prompt { message } if !message.trim().is_empty() => Ok(message),
        ClientFrame::Prompt { .. } => Err("Prompt text is empty.".into()),
        ClientFrame::Steer { .. } | ClientFrame::Cancel => {
            Err("The first realtime frame must be a prompt.".into())
        }
    }
}
