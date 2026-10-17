//! Voice transport events end.
use super::*;
pub(super) fn event(room_name: &String, reason: &String) -> proto::VoiceEvent {
    proto::VoiceEvent {
        event: Some(proto::voice_event::Event::SessionEnded(
            proto::VoiceSessionEndedEvent {
                room_name: room_name.clone(),
                reason: reason.clone(),
            },
        )),
    }
}
