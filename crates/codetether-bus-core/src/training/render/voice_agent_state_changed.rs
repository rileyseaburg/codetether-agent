//! VoiceAgentStateChanged training record projection.
use super::*;
pub(super) fn build(room_name: &String, state: &String, meta: TrainingMetadata) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!("Voice agent state: room={room_name} → {state}")),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
