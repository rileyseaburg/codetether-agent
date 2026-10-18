//! In-turn steering and cancellation command handling.

use super::frames::{ClientFrame, ServerFrame};
use super::send;
use super::socket::SocketSink;

/// Whether the active turn should continue after one client command.
/// # Examples
/// ```
/// let _frame = codetether_session_realtime::turn_input::InputAction::Continue;
/// ```
pub enum InputAction {
    Continue,
    Cancel,
}

/// Apply one client command at the active prompt boundary.
/// # Arguments
/// * `sink` — Sending half.
/// * `session_id` — Active session identity.
/// * `frame` — Incoming command.
/// # Returns
/// Continue after steering, or Cancel for cancellation.
/// # Errors
/// Returns a steering/send failure or an error for a second prompt.
/// # Examples
/// ```no_run
/// # async fn example(sink: &mut codetether_session_realtime::socket::SocketSink, id: &str, frame: codetether_session_realtime::frames::ClientFrame) {
/// let _ = codetether_session_realtime::turn_input::handle(sink, id, frame).await;
/// # }
/// ```
pub async fn handle(
    sink: &mut SocketSink,
    session_id: &str,
    frame: ClientFrame,
) -> Result<InputAction, String> {
    match frame {
        ClientFrame::Steer {
            request_id,
            message,
        } => {
            let accepted = codetether_session_steering::send(session_id, &message)
                .await
                .map_err(|error| error.to_string())?;
            send::frame(
                sink,
                &ServerFrame::Steering {
                    request_id,
                    accepted,
                },
            )
            .await?;
            Ok(InputAction::Continue)
        }
        ClientFrame::Cancel => Ok(InputAction::Cancel),
        ClientFrame::Prompt { .. } => Err("A prompt is already active on this connection.".into()),
    }
}
