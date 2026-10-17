//! Default CodeTether agent-card metadata and coding skills.
//!
//! [default_card] builds the advertised wire record. Callers provide their
//! application version and can customize identity, name, or description.
use crate::{AgentCapabilities, AgentCard, AgentProvider};
mod skills;

/// Create a default agent card
///
/// # Arguments
/// * `url` - Public base URL advertised by the peer.
/// * `version` - Application version published in the card.
/// # Returns
/// The default CodeTether card with coding skills and transport capabilities.
/// # Examples
/// ```rust
/// use codetether_a2a_core::application_card::default_card;
/// let card = default_card("http://localhost:4096", "1.0.0");
/// assert_eq!(card.version, "1.0.0");
/// assert_eq!(card.skills.len(), 3);
/// ```
pub fn default_card(url: &str, version: &str) -> AgentCard {
    AgentCard {
        name: "CodeTether Agent".to_string(),
        description: "A2A-native AI coding agent for the CodeTether ecosystem".to_string(),
        url: url.to_string(),
        version: version.to_string(),
        protocol_version: "0.3.0".to_string(),
        preferred_transport: None,
        additional_interfaces: vec![],
        capabilities: AgentCapabilities {
            streaming: true,
            push_notifications: false,
            state_transition_history: true,
            extensions: vec![],
        },
        skills: skills::skills(),
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
