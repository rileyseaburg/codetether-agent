//! A2A message construction for automatic peer introductions.

use crate::a2a::types::{Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part};

pub(super) fn build(agent_name: &str, self_url: &str) -> MessageSendParams {
    let mut metadata = std::collections::HashMap::new();
    metadata.insert(
        super::detect::INTRO_METADATA_KEY.to_string(),
        serde_json::Value::Bool(true),
    );
    MessageSendParams {
        message: Message {
            message_id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::User,
            parts: vec![Part::Text {
                text: format!(
                    "Hello from {agent_name} ({self_url}). I am online and available for A2A collaboration."
                ),
            }],
            context_id: None,
            task_id: None,
            metadata,
            extensions: vec![],
        },
        configuration: Some(MessageSendConfiguration {
            accepted_output_modes: vec!["text/plain".to_string()],
            blocking: Some(false),
            history_length: Some(0),
            push_notification_config: None,
        }),
    }
}
