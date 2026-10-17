//! Voice transport create voice session.
use super::*;
pub(super) async fn create_voice_session(
    service: &VoiceServiceImpl,
    request: Request<proto::CreateVoiceSessionRequest>,
) -> Result<Response<proto::VoiceSession>, Status> {
    let req = request.into_inner();
    let voice_id = if req.voice_id.is_empty() {
        "960f89fc".to_string()
    } else {
        req.voice_id
    };

    let room_name = format!("voice-{}", uuid::Uuid::new_v4());

    // Publish to bus
    let handle = service.bus.handle("voice-grpc");
    handle.send_voice_session_started(&room_name, &voice_id);

    let session = proto::VoiceSession {
        room_name,
        voice_id,
        state: proto::VoiceSessionState::Creating.into(),
        agent_state: proto::VoiceAgentState::Initializing.into(),
        access_token: String::new(), // populated by LiveKit bridge
        livekit_url: std::env::var("LIVEKIT_URL").unwrap_or_default(),
        created_at: Some(prost_types::Timestamp {
            seconds: chrono::Utc::now().timestamp(),
            nanos: 0,
        }),
    };

    Ok(Response::new(session))
}
