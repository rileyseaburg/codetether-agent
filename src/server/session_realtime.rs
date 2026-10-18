//! Authenticated realtime transport for one active session turn.

#[path = "session_realtime/prompt_run.rs"]
mod prompt_run;

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
        .on_upgrade(move |websocket| {
            codetether_session_realtime::run(websocket, id, prompt_run::start)
        })
}
