//! Non-blocking A2A request construction for a remote agent turn.

use codetether_a2a_core::{
    Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part,
};

pub(super) fn build(text: &str, context_id: Option<&str>) -> MessageSendParams {
    MessageSendParams {
        message: Message {
            message_id: uuid::Uuid::new_v4().to_string(),
            role: MessageRole::User,
            parts: vec![Part::Text {
                text: text.to_string(),
            }],
            context_id: context_id.map(ToString::to_string),
            task_id: None,
            metadata: metadata(),
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

#[cfg(test)]
#[path = "payload_tests.rs"]
mod tests;

/// Transport marker plus our own peer name so the receiver can say who asked.
fn metadata() -> std::collections::HashMap<String, serde_json::Value> {
    let mut map = std::collections::HashMap::from([(
        "codetether.transport".to_string(),
        serde_json::Value::String("mdns".to_string()),
    )]);
    if let Some(name) = codetether_a2a_discovery::local_identity::current_name() {
        map.insert(
            "codetether.sender".to_string(),
            serde_json::Value::String(name),
        );
    }
    map
}
