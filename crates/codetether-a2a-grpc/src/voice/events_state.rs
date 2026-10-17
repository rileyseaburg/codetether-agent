//! Voice transport events state.
use super::*;
pub(super) fn event(room_name: &String, state: &String) -> proto::VoiceEvent {
    let proto_state = match state.as_str() {
        "listening" => proto::VoiceAgentState::Listening,
        "thinking" => proto::VoiceAgentState::Thinking,
        "speaking" => proto::VoiceAgentState::Speaking,
        _ => proto::VoiceAgentState::Initializing,
    };

    proto::VoiceEvent {
        event: Some(proto::voice_event::Event::AgentStateChange(
            proto::VoiceAgentStateEvent {
                room_name: room_name.clone(),
                state: proto_state.into(),
            },
        )),
    }
}
