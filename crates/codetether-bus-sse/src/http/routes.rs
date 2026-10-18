//! General bus event stream route assembly.
use super::{HttpState, handler};
use axum::{Router, routing::get};

/// Build the bus event stream route for the caller's application state.
/// # Returns
/// The SSE route to merge beneath authentication, policy and audit layers.
/// # Examples
/// ```no_run
/// # fn example<S: codetether_bus_sse::http::HttpState>() {
/// let routes = codetether_bus_sse::http::router::<S>();
/// # }
/// ```
pub fn router<S: HttpState>() -> Router<S> {
    Router::new().route("/v1/bus/stream", get(handler::stream_bus_events::<S>))
}
