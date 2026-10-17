//! send voice transcript operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Publish a transcript fragment from a voice session.
    pub fn send_voice_transcript(
        &self,
        room_name: &str,
        text: &str,
        role: &str,
        is_final: bool,
    ) -> usize {
        self.send(
            format!("voice.{room_name}"),
            BusMessage::VoiceTranscript {
                room_name: room_name.to_string(),
                text: text.to_string(),
                role: role.to_string(),
                is_final,
            },
        )
    }
}
