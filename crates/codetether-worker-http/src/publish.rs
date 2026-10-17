//! Publish supplied payloads onto the local bus.
use super::*;

pub(super) async fn publish_bus_event(
    State(state): State<WorkerServerState>,
    Json(req): Json<BusPublishRequest>,
) -> StatusCode {
    let bus = state.bus.lock().await.clone();
    let Some(bus) = bus else {
        return StatusCode::SERVICE_UNAVAILABLE;
    };
    let handle = bus.handle("worker_server_publish");
    handle.send(
        &req.topic,
        codetether_bus_core::BusMessage::SharedResult {
            key: req.topic.clone(),
            value: req.payload,
            tags: vec![],
        },
    );
    StatusCode::ACCEPTED
}
