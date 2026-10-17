use super::*;
pub(super) fn build(agent_id: &str, skills: Vec<AgentSkill>) -> AgentCard {
    let card = AgentCard {
        name: agent_id.to_string(),
        description: format!("In-process agent {agent_id} (registered via AgentReady)"),
        url: format!("bus://local/{agent_id}"),
        version: "ephemeral".to_string(),
        protocol_version: "0.3.0".to_string(),
        preferred_transport: Some("BUS".to_string()),
        additional_interfaces: vec![],
        capabilities: AgentCapabilities {
            streaming: true,
            push_notifications: false,
            state_transition_history: false,
            extensions: vec![],
        },
        skills,
        default_input_modes: vec!["text".to_string()],
        default_output_modes: vec!["text".to_string()],
        provider: None,
        icon_url: None,
        documentation_url: None,
        security_schemes: Default::default(),
        security: vec![],
        supports_authenticated_extended_card: false,
        signatures: vec![],
    };

    card
}
