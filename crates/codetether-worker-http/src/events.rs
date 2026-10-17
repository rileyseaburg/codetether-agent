//! Map bus reception outcomes to SSE data, keepalive and lag events.
use super::*;
pub(super) fn events(
    rx: broadcast::Receiver<BusEnvelope>,
    topic_filter: Option<String>,
) -> impl futures::Stream<Item = Result<Event, Infallible>> {
    stream::unfold(rx, move |mut rx| {
        let filter = topic_filter.clone();
        async move {
            match rx.recv().await {
                Ok(envelope) => {
                    let allowed = filter
                        .as_deref()
                        .map(|pat| bus_topic_matches(&envelope.topic, pat))
                        .unwrap_or(true);

                    if allowed {
                        let payload =
                            serde_json::to_string(&envelope).unwrap_or_else(|_| "{}".to_string());
                        Some((
                            Ok::<Event, Infallible>(Event::default().event("bus").data(payload)),
                            rx,
                        ))
                    } else {
                        Some((
                            Ok::<Event, Infallible>(Event::default().event("keepalive").data("")),
                            rx,
                        ))
                    }
                }
                Err(broadcast::error::RecvError::Lagged(n)) => Some((
                    Ok(Event::default().event("lag").data(format!("skipped {}", n))),
                    rx,
                )),
                Err(broadcast::error::RecvError::Closed) => None,
            }
        }
    })
}
