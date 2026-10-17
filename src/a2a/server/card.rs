//! Application default agent-card metadata.
use super::*;
impl A2AServer {
    /// Create a default agent card
    pub fn default_card(url: &str) -> AgentCard {
        AgentCard {
            name: "CodeTether Agent".to_string(),
            description: "A2A-native AI coding agent for the CodeTether ecosystem".to_string(),
            url: url.to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            protocol_version: "0.3.0".to_string(),
            preferred_transport: None,
            additional_interfaces: vec![],
            capabilities: AgentCapabilities {
                streaming: true,
                push_notifications: false,
                state_transition_history: true,
                extensions: vec![],
            },
            skills: card_skills::skills(),
            default_input_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            default_output_modes: vec!["text/plain".to_string(), "application/json".to_string()],
            provider: Some(AgentProvider {
                organization: "CodeTether".to_string(),
                url: "https://codetether.run".to_string(),
            }),
            icon_url: None,
            documentation_url: None,
            security_schemes: Default::default(),
            security: vec![],
            supports_authenticated_extended_card: false,
            signatures: vec![],
        }
    }
}
