//! VoiceSessionStarted training record projection.
use super::*;
pub(super) fn build(
    room_name: &String,
    agent_id: &String,
    voice_id: &String,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Voice session started: room={room_name}, agent={agent_id}, voice={voice_id}"
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
