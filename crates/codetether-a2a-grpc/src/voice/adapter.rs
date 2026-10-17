//! Voice transport adapter.
use super::*;
implement_service! {
    create_voice_session: proto::CreateVoiceSessionRequest => proto::VoiceSession,
    get_voice_session: proto::GetVoiceSessionRequest => proto::VoiceSession,
    delete_voice_session: proto::DeleteVoiceSessionRequest => (),
    list_voices: proto::ListVoicesRequest => proto::ListVoicesResponse,
    stream_voice_events: proto::StreamVoiceEventsRequest => StreamResult<proto::VoiceEvent>,
}
