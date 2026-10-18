//! JWT topic extraction, bus subscription and SSE transport.
use super::HttpState;
use axum::{
    body::Body,
    extract::State,
    http::Request,
    response::sse::{Event, KeepAlive, Sse},
};
use std::convert::Infallible;

/// Stream bus events as SSE, filtered by JWT topic claims.
///
/// The JWT must contain a `topics` claim with an array of topic patterns
/// to subscribe to. Events are filtered to only include envelopes whose
/// topic matches one of the allowed patterns.
pub(super) async fn stream_bus_events<S: HttpState>(
    State(state): State<S>,
    req: Request<Body>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    // Extract JWT claims from request extensions (set by auth middleware)
    let allowed_topics: Vec<String> = req
        .extensions()
        .get::<codetether_http_auth::JwtClaims>()
        .map(|claims| claims.topics.clone())
        .unwrap_or_default();

    // Subscribe to the bus
    let bus_handle = state.bus().handle("stream_bus_events");
    let rx = bus_handle.into_receiver();

    let event_stream = crate::bus_events(rx, allowed_topics);

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
}
