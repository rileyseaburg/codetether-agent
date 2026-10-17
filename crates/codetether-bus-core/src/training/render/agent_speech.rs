//! AgentSpeech training record projection.
use super::*;
pub(super) fn build(
    act: &String,
    from: &String,
    to: &String,
    conversation_id: &String,
    content: &String,
    meta: TrainingMetadata,
) -> TrainingRecord {
    speech::build(act, from, to, conversation_id, content, meta)
}
