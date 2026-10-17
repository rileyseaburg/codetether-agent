//! local card to proto conversion.
use super::*;

/// Convert a local `AgentCard` to a proto `AgentCard`.
pub fn local_card_to_proto(card: &local::AgentCard) -> proto::AgentCard {
    proto::AgentCard {
        protocol_version: card.protocol_version.clone(),
        name: card.name.clone(),
        description: card.description.clone(),
        url: card.url.clone(),
        preferred_transport: card.preferred_transport.clone().unwrap_or_default(),
        additional_interfaces: card_interfaces::interfaces(card),
        provider: card.provider.as_ref().map(|p| proto::AgentProvider {
            url: p.url.clone(),
            organization: p.organization.clone(),
        }),
        version: card.version.clone(),
        documentation_url: card.documentation_url.clone().unwrap_or_default(),
        capabilities: card_capabilities::capabilities(card),
        security_schemes: Default::default(), // complex mapping deferred
        security: vec![],
        default_input_modes: card.default_input_modes.clone(),
        default_output_modes: card.default_output_modes.clone(),
        skills: card_skills::skills(card),
        supports_authenticated_extended_card: card.supports_authenticated_extended_card,
        signatures: card_signatures::signatures(card),
        icon_url: card.icon_url.clone().unwrap_or_default(),
    }
}
