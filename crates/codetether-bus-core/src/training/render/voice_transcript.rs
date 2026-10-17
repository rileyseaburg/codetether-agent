//! VoiceTranscript training record projection.
use super::*;
pub(super) fn build(
    room_name: &String,
    text: &String,
    role: &String,
    is_final: &bool,
    meta: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: if role == "user" {
            "user".into()
        } else {
            "assistant".into()
        },
        content: Some(format!(
            "[voice:{room_name}{}] {text}",
            if *is_final { " final" } else { "" }
        )),
        tool_calls: None,
        tool_call_id: None,
        name: None,
        metadata: meta,
    }
}
