//! Cognition events, explicit lag notification and keepalive transport.
use axum::response::sse::{Event, KeepAlive, Sse};
use futures::stream;
use std::convert::Infallible;
/// Stream an existing cognition subscription until its sender closes.
/// # Arguments
/// * `rx` - Runtime event subscription.
/// # Returns
/// SSE response with cognition events, lag messages and 15-second keepalives.
/// # Examples
/// ```
/// let (_tx, rx) = tokio::sync::broadcast::channel(4);
/// let _response = codetether_cognition_api::stream_cognition(rx);
/// ```

pub fn stream_cognition(
    rx: tokio::sync::broadcast::Receiver<codetether_cognition_core::ThoughtEvent>,
) -> Sse<impl futures::Stream<Item = Result<Event, Infallible>>> {
    let event_stream = stream::unfold(rx, |mut rx| async move {
        match rx.recv().await {
            Ok(event) => {
                let payload = serde_json::to_string(&event).unwrap_or_else(|_| "{}".to_string());
                let sse_event = Event::default().event("cognition").data(payload);
                Some((Ok(sse_event), rx))
            }
            Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                let lag_event = Event::default()
                    .event("lag")
                    .data(format!("skipped {}", skipped));
                Some((Ok(lag_event), rx))
            }
            Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
        }
    });

    Sse::new(event_stream).keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
}
