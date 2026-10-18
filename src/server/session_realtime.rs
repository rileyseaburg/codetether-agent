//! Authenticated realtime transport for one active session turn.

#[path = "session_realtime/connection.rs"]
mod connection;
#[path = "session_realtime/prompt_run.rs"]
mod prompt_run;
#[path = "session_realtime/turn.rs"]
mod turn;
#[path = "session_realtime/turn_start.rs"]
mod turn_start;
#[path = "session_realtime/turn_step.rs"]
mod turn_step;

use axum::extract::Path;
use axum::extract::ws::WebSocketUpgrade;
use axum::response::Response;
use codetether_session_realtime::{
    event_forward, frames, handshake, send, socket, turn_event, turn_finish, turn_input, wire,
};

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
