//! Voice transport delete voice session.
use super::*;
pub(super) async fn delete_voice_session(
    service: &VoiceServiceImpl,
    request: Request<proto::DeleteVoiceSessionRequest>,
) -> Result<Response<()>, Status> {
    let req = request.into_inner();
    if req.room_name.is_empty() {
        return Err(Status::invalid_argument("room_name is required"));
    }

    let handle = service.bus.handle("voice-grpc");
    handle.send_voice_session_ended(&req.room_name, "user_ended");

    Ok(Response::new(()))
}
