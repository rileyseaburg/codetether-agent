//! send voice session started operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Announce that a voice session has started.
    pub fn send_voice_session_started(&self, room_name: &str, voice_id: &str) -> usize {
        self.send(
            format!("voice.{room_name}"),
            BusMessage::VoiceSessionStarted {
                room_name: room_name.to_string(),
                agent_id: self.agent_id().to_string(),
                voice_id: voice_id.to_string(),
            },
        )
    }
}
