//! VoiceSessionEnded training record projection.
use super::*;
pub(super) fn build(room_name: &String, reason: &String, meta: TrainingMetadata) -> TrainingRecord {
    TrainingRecord {
        role: "system".into(),
        content: Some(format!(
            "Voice session ended: room={room_name}, reason={reason}"
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
