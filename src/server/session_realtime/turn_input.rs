//! In-turn steering and cancellation command handling.

use super::frames::{ClientFrame, ServerFrame};
use super::send;
use super::socket::SocketSink;

/// Whether the active turn should continue after one client command.
pub(super) enum InputAction {
    Continue,
    Cancel,
}

/// Apply one client command at the active prompt boundary.
pub(super) async fn handle(
    sink: &mut SocketSink,
    session_id: &str,
    frame: ClientFrame,
) -> Result<InputAction, String> {
    match frame {
        ClientFrame::Steer {
            request_id,
            message,
        } => {
            let accepted = crate::session::helper::steering::send(session_id, &message)
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
