//! Voice transport events transcript.
use super::*;
pub(super) fn event(
    room_name: &String,
    text: &String,
    role: &String,
    is_final: bool,
    timestamp: &chrono::DateTime<chrono::Utc>,
) -> proto::VoiceEvent {
    let proto_role = match role.as_str() {
        "user" => proto::Role::User,
        _ => proto::Role::Agent,
    };

    proto::VoiceEvent {
        event: Some(proto::voice_event::Event::Transcript(
            proto::VoiceTranscriptEvent {
                room_name: room_name.clone(),
                text: text.clone(),
                role: proto_role.into(),
                is_final: is_final,
                timestamp: Some(prost_types::Timestamp {
                    seconds: timestamp.timestamp(),
                    nanos: timestamp.timestamp_subsec_nanos() as i32,
                }),
            },
        )),
    }
}
