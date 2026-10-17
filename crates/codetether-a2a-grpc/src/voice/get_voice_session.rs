//! Voice transport get voice session.
use super::*;
pub(super) async fn get_voice_session(
    _service: &VoiceServiceImpl,
    request: Request<proto::GetVoiceSessionRequest>,
) -> Result<Response<proto::VoiceSession>, Status> {
    let req = request.into_inner();
    if req.room_name.is_empty() {
        return Err(Status::invalid_argument("room_name is required"));
    }

    // Return a stub session — the Python LiveKit bridge is the source
    // of truth. This endpoint is a pass-through for the dashboard to
    // get the current state via gRPC instead of REST.
    Ok(Response::new(proto::VoiceSession {
        room_name: req.room_name,
        voice_id: String::new(),
        state: proto::VoiceSessionState::Active.into(),
        agent_state: proto::VoiceAgentState::Listening.into(),
        access_token: String::new(),
        livekit_url: std::env::var("LIVEKIT_URL").unwrap_or_default(),
        created_at: None,
    }))
}
