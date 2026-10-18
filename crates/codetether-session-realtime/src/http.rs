//! WebSocket upgrade route for authenticated session turns.
use crate::Host;
use axum::{
    Router,
    extract::{Path, ws::WebSocketUpgrade},
    response::Response,
    routing::get,
};
/// Build the realtime session WebSocket route.
/// # Returns
/// The upgrade route to merge beneath the server's authentication layers.
/// # Examples
/// ```no_run
/// # fn example<H: codetether_session_realtime::Host>() {
/// let routes = codetether_session_realtime::http::router::<H, ()>();
/// # }
/// ```
pub fn router<H: Host, S: Clone + Send + Sync + 'static>() -> Router<S> {
    Router::new().route("/api/realtime/session/{id}", get(upgrade::<H>))
}

async fn upgrade<H: Host>(Path(id): Path<String>, socket: WebSocketUpgrade) -> Response {
    socket
        .max_message_size(64 * 1024)
        .on_upgrade(move |websocket| crate::run(websocket, id, crate::execution::start::<H>))
}
