//! Build the stream reply message after persistence.
use super::*;
pub(super) fn message(
    task_id: &String,
    context_id: Option<String>,
    result_text: &String,
) -> Message {
    Message {
        message_id: Uuid::new_v4().to_string(),
        role: MessageRole::Agent,
        parts: vec![Part::Text {
            text: result_text.clone(),
        }],
        context_id,
        task_id: Some(task_id.clone()),
        metadata: std::collections::HashMap::new(),
        extensions: vec![],
    }
}
