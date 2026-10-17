//! Agent-card capabilities projection.
use super::*;
pub(super) fn capabilities(card: &local::AgentCard) -> Option<proto::AgentCapabilities> {
    Some(proto::AgentCapabilities {
        streaming: card.capabilities.streaming,
        push_notifications: card.capabilities.push_notifications,
        extensions: card
            .capabilities
            .extensions
            .iter()
            .map(|e| proto::AgentExtension {
                uri: e.uri.clone(),
                description: e.description.clone().unwrap_or_default(),
                required: e.required,
                params: None,
            })
            .collect(),
    })
}
