//! Outbound mesh introduction: one-shot, tagged, ledger-deduplicated.

use crate::a2a::client::A2AClient;
use crate::a2a::types::{Message, MessageRole, MessageSendConfiguration, MessageSendParams, Part};
use anyhow::Result;

use super::detect::INTRO_METADATA_KEY;
use super::ledger;

/// Send a non-blocking intro `message/send` to `endpoint`, unless the
/// persistent ledger says we already introduced ourselves there.
///
/// The message carries `metadata[codetether.intro] = true` so receivers
/// can short-circuit it without an LLM call.
pub async fn send_intro(endpoint: &str, agent_name: &str, self_url: &str) -> Result<()> {
    if ledger::contains(endpoint) {
        tracing::debug!(peer = %endpoint, "Skipping intro: already in ledger");
        return Ok(());
    }

    let mut client = A2AClient::new(endpoint);
    if let Ok(token) = std::env::var("CODETETHER_AUTH_TOKEN") {
        client = client.with_token(token);
    }
    let mut metadata = std::collections::HashMap::new();
    metadata.insert(
        INTRO_METADATA_KEY.to_string(),
        serde_json::Value::Bool(true),
    );
    let payload = MessageSendParams {
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
    };

    let _ = client.send_message(payload).await?;
    ledger::record(endpoint);
    tracing::info!(peer = %endpoint, "Auto-intro message sent");
    Ok(())
}
