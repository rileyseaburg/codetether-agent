//! Voice transport stream voice events.
use super::*;
pub(super) async fn stream_voice_events(
    service: &VoiceServiceImpl,
    request: Request<proto::StreamVoiceEventsRequest>,
) -> Result<Response<StreamResult<proto::VoiceEvent>>, Status> {
    let req = request.into_inner();
    if req.room_name.is_empty() {
        return Err(Status::invalid_argument("room_name is required"));
    }

    let room_name = req.room_name;
    let bus_handle = service.bus.handle("voice-stream");
    let rx = bus_handle.into_receiver();
    let topic_prefix = format!("voice.{room_name}");

    let stream = stream::stream(rx, room_name, topic_prefix);

    Ok(Response::new(stream))
}
