//! Authenticated realtime transport for one active session turn.

#[path = "session_realtime/connection.rs"]
mod connection;
#[path = "session_realtime/event_forward.rs"]
mod event_forward;
#[path = "session_realtime/frames.rs"]
mod frames;
#[path = "session_realtime/handshake.rs"]
mod handshake;
#[path = "session_realtime/prompt_run.rs"]
mod prompt_run;
#[path = "session_realtime/send.rs"]
mod send;
#[path = "session_realtime/socket.rs"]
mod socket;
#[path = "session_realtime/turn.rs"]
mod turn;
#[path = "session_realtime/turn_event.rs"]
mod turn_event;
#[path = "session_realtime/turn_finish.rs"]
mod turn_finish;
#[path = "session_realtime/turn_input.rs"]
mod turn_input;
#[path = "session_realtime/turn_start.rs"]
mod turn_start;
#[path = "session_realtime/turn_step.rs"]
mod turn_step;
#[path = "session_realtime/wire.rs"]
mod wire;

use axum::extract::Path;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::Response;

/// Build the authenticated WebSocket route for session turns.
pub(super) fn router() -> axum::Router<super::AppState> {
    use axum::routing::get;

    axum::Router::new().route("/api/realtime/session/{id}", get(upgrade))
}

/// Upgrade one authenticated request into a bounded realtime connection.
async fn upgrade(Path(id): Path<String>, socket: WebSocketUpgrade) -> Response {
    socket
        .max_message_size(64 * 1024)
        .on_upgrade(move |websocket| connection::run(websocket, id))
}

#[cfg(test)]
#[path = "session_realtime/tests.rs"]
mod tests;
