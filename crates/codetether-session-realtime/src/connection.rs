//! WebSocket connection setup before one prompt turn begins.

use axum::extract::ws::WebSocket;
use futures::StreamExt;

use super::frames::ServerFrame;
use super::{handshake, send, turn};

/// Announce the session, validate the prompt, and enter its turn loop.
/// # Arguments
/// * `socket` - Authenticated upgraded connection.
/// * `session_id` - Session identity announced to the client.
/// * `start` - Host callback that starts a prompt with its event channel.
/// # Returns
/// Completes after the turn finishes, is cancelled, or the connection fails.
/// # Examples
/// ```no_run
/// # async fn example(socket: axum::extract::ws::WebSocket, start: codetether_session_realtime::PromptStarter<()>) {
/// codetether_session_realtime::run(socket, "session-1".into(), start).await;
/// # }
/// ```
pub async fn run<S: Send + Sync + 'static>(
    socket: WebSocket,
    session_id: String,
    start: super::PromptStarter<S>,
) {
    let (mut sink, mut stream) = socket.split();
    let ready = ServerFrame::Ready {
        session_id: session_id.clone(),
        model_selection: true,
    };
    if send::frame(&mut sink, &ready).await.is_err() {
        return;
    }
    match handshake::prompt(&mut stream).await {
        Ok((message, model)) => turn::run(sink, stream, session_id, message, model, start).await,
        Err(message) => {
            let _ = send::frame(&mut sink, &ServerFrame::Error { message }).await;
        }
    }
}
