//! Build a training record from an `AgentSpeech` bus message.

use super::{TrainingMetadata, TrainingRecord};

/// Convert an agent speech act into an assistant-role training record.
pub(super) fn build(
    act: &str,
    from: &str,
    to: &str,
    conversation_id: &str,
    content: &str,
    metadata: TrainingMetadata,
) -> TrainingRecord {
    TrainingRecord {
        role: "assistant".into(),
        content: Some(format!(
            "[{act}] {from}→{to} (conv {conversation_id}): {content}"
        )),
        tool_calls: None,
        tool_call_id: None,
        name: Some(from.to_string()),
        metadata,
    }
}
