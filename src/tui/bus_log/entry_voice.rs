//! Router for voice session entries.

use ratatui::style::Color;

use crate::bus::BusMessage;

use super::{entry_parts::EntryParts, entry_voice_transcript};

pub(super) fn entry_parts(message: &BusMessage) -> EntryParts {
    if let BusMessage::VoiceSessionStarted {
        room_name,
        agent_id,
        voice_id,
    } = message
    {
        return EntryParts::new(
            "VOICE+",
            format!("{room_name} agent={agent_id} voice={voice_id}"),
            format!("Room: {room_name}\nAgent: {agent_id}\nVoice: {voice_id}"),
            Color::LightCyan,
        );
    }
    if let BusMessage::VoiceTranscript {
        room_name,
        text,
        role,
        is_final,
    } = message
    {
        return entry_voice_transcript::transcript(room_name, text, role, *is_final);
    }
    if let BusMessage::VoiceAgentStateChanged { room_name, state } = message {
        return EntryParts::new(
            "VOICE•S",
            format!("{room_name} → {state}"),
            format!("Room: {room_name}\nState: {state}"),
            Color::LightCyan,
        );
    }
    if let BusMessage::VoiceSessionEnded { room_name, reason } = message {
        return EntryParts::new(
            "VOICE-",
            format!("{room_name} ended: {reason}"),
            format!("Room: {room_name}\nReason: {reason}"),
            Color::DarkGray,
        );
    }
    unreachable!("voice entry builder received a non-voice bus message");
}
