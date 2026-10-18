//! Rust-owned HTTP surface for authoritative mux roster and realtime output.

mod session_projection;
mod sessions;
mod stream;

/// Construct mux roster and live-output routes under caller-owned middleware.
/// # Returns
/// A router exposing session roster JSON and live mux output SSE.
/// # Examples
/// ```
/// let _router = codetether_mux_http::router::<()>();
/// ```
pub fn router<S: Clone + Send + Sync + 'static>() -> axum::Router<S> {
    use axum::routing::get;

    axum::Router::new()
        .route("/v1/bus/stream/mux/sessions", get(sessions::list))
        .route("/v1/bus/stream/mux", get(stream::output))
}
