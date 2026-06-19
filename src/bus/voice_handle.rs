//! Voice-session [`BusHandle`] helpers, split out for SRP / line budget.

use super::{BusHandle, BusMessage};

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
