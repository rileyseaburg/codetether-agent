//! WebSocket connection setup before one prompt turn begins.

use axum::extract::ws::WebSocket;
use futures::StreamExt;

use super::frames::ServerFrame;
use super::{handshake, send, turn};

/// Announce the session, validate the prompt, and enter its turn loop.
pub(super) async fn run(socket: WebSocket, session_id: String) {
    let (mut sink, mut stream) = socket.split();
    let ready = ServerFrame::Ready {
        session_id: session_id.clone(),
    };
    if send::frame(&mut sink, &ready).await.is_err() {
        return;
    }
    match handshake::prompt(&mut stream).await {
        Ok(message) => turn::run(sink, stream, session_id, message).await,
        Err(message) => {
            let _ = send::frame(&mut sink, &ServerFrame::Error { message }).await;
        }
    }
}
