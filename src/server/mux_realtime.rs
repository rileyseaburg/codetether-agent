//! Rust-owned HTTP surface for authoritative mux roster and realtime output.

#[path = "mux_realtime/session_projection.rs"]
mod session_projection;
#[path = "mux_realtime/sessions.rs"]
mod sessions;
#[path = "mux_realtime/stream.rs"]
mod stream;

pub(super) fn router() -> axum::Router<super::super::AppState> {
    use axum::routing::get;

    axum::Router::new()
        .route("/v1/bus/stream/mux/sessions", get(sessions::list))
        .route("/v1/bus/stream/mux", get(stream::output))
}
