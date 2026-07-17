//! Blocking A2A request construction for a remote agent turn.

use crate::a2a::types::{Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part};

pub(super) fn build(text: &str) -> MessageSendParams {
    MessageSendParams {
        message: Message {
            message_id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::User,
            parts: vec![Part::Text {
                text: text.to_string(),
            }],
            context_id: None,
            task_id: None,
            metadata: std::collections::HashMap::from([(
                "codetether.transport".to_string(),
                serde_json::Value::String("mdns".to_string()),
            )]),
            extensions: vec![],
        },
        configuration: Some(MessageSendConfiguration {
            accepted_output_modes: vec!["text/plain".to_string()],
            blocking: Some(true),
            history_length: Some(0),
            push_notification_config: None,
        }),
    }
}
