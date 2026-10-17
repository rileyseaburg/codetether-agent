//! send voice agent state operations for the shared bus runtime.
use super::*;
impl BusHandle {
    /// Announce a voice agent state change.
    pub fn send_voice_agent_state(&self, room_name: &str, state: &str) -> usize {
        self.send(
            format!("voice.{room_name}"),
            BusMessage::VoiceAgentStateChanged {
                room_name: room_name.to_string(),
                state: state.to_string(),
            },
        )
    }
}
