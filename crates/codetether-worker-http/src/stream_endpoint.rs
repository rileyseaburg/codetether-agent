//! Subscribe to the attached bus and construct its HTTP event stream.
use super::*;

/// SSE stream of agent bus events — subscribe to live bus messages.
/// Streams all topics; use the `topic` query param to filter (e.g. `?topic=task.*`).
pub(super) async fn stream_bus_events(
    State(state): State<WorkerServerState>,
    req: Request<Body>,
) -> Response {
    let bus = state.bus.lock().await.clone();
    let Some(bus) = bus else {
        // Bus not attached — return an empty keep-alive stream
        let empty = stream::empty::<Result<Event, Infallible>>();
        return Sse::new(empty)
            .keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
            .into_response();
    };

    let topic_filter: Option<String> = req.uri().query().and_then(|q| {
        q.split('&')
            .filter_map(|pair| pair.split_once('='))
            .find(|(k, _)| *k == "topic")
            .map(|(_, v)| v.to_owned())
    });

    let bus_handle = bus.handle("worker_server_bus_stream");
    let rx: broadcast::Receiver<BusEnvelope> = bus_handle.into_receiver();

    let event_stream = events(rx, topic_filter);

    Sse::new(event_stream)
        .keep_alive(KeepAlive::new().interval(std::time::Duration::from_secs(15)))
        .into_response()
}
