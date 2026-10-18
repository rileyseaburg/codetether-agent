//! Topic-filtered general bus SSE stream.
use crate::topic::topic_matches;
use axum::response::sse::Event;
use codetether_bus_core::BusEnvelope;
use futures::{Stream, stream};
use std::convert::Infallible;
use tokio::sync::broadcast;
/// Stream bus envelopes allowed by the supplied topic patterns.
/// # Arguments
/// * `rx` - Subscription receiver.
/// * `allowed_topics` - Topic patterns; an empty list permits every envelope.
/// # Returns
/// Bus events, keepalives for filtered envelopes, and lag notices until closure.
/// # Examples
/// ```
/// let (_, rx) = tokio::sync::broadcast::channel(8);
/// let _stream = codetether_bus_sse::bus_events(rx, vec![]);
/// ```
pub fn events(
    rx: broadcast::Receiver<BusEnvelope>,
    allowed_topics: Vec<String>,
) -> impl Stream<Item = Result<Event, Infallible>> {
    stream::unfold(rx, move |mut rx: broadcast::Receiver<BusEnvelope>| {
        let allowed_topics = allowed_topics.clone();
        async move {
            match rx.recv().await {
                Ok(envelope) => {
                    // Filter by allowed topics if any are specified
                    let should_send = allowed_topics.is_empty()
                        || allowed_topics
                            .iter()
                            .any(|pattern| topic_matches(&envelope.topic, pattern));

                    if should_send {
                        let payload =
                            serde_json::to_string(&envelope).unwrap_or_else(|_| "{}".to_string());
                        let sse_event = Event::default().event("bus").data(payload);
                        return Some((Ok(sse_event), rx));
                    }

                    // Skip this event but keep the receiver
                    Some((Ok(Event::default().event("keepalive").data("")), rx))
                }
                Err(tokio::sync::broadcast::error::RecvError::Lagged(skipped)) => {
                    let lag_event = Event::default()
                        .event("lag")
                        .data(format!("skipped {}", skipped));
                    Some((Ok(lag_event), rx))
                }
                Err(tokio::sync::broadcast::error::RecvError::Closed) => None,
            }
        }
    })
}
