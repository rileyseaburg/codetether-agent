//! agent message task operation.
use super::*;

pub fn agent_message(task_id: &str, context_id: Option<&str>, text: String) -> Message {
    Message {
        message_id: Uuid::new_v4().to_string(),
        role: MessageRole::Agent,
        parts: vec![Part::Text { text }],
        context_id: context_id.map(ToString::to_string),
        task_id: Some(task_id.to_string()),
        metadata: Default::default(),
        extensions: vec![],
    }
}
