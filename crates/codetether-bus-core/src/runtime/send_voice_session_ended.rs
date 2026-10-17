//! send voice session ended operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Announce that a voice session has ended.
    pub fn send_voice_session_ended(&self, room_name: &str, reason: &str) -> usize {
        self.send(
            format!("voice.{room_name}"),
            BusMessage::VoiceSessionEnded {
                room_name: room_name.to_string(),
                reason: reason.to_string(),
            },
        )
    }
}
