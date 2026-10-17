use super::*;
impl AgentRegistry {
    /// Create and register an ephemeral card for a sub-agent.
    ///
    /// These cards are lightweight and intended for in-process sub-agents
    /// that exist only for the lifetime of a swarm execution.  The URL
    /// is set to `bus://local/{agent_id}` to signal that the agent is only
    /// reachable through the in-process bus.
    pub fn create_ephemeral(
        &self,
        name: impl Into<String>,
        description: impl Into<String>,
        skills: Vec<AgentSkill>,
    ) -> AgentCard {
        let name = name.into();
        let card = AgentCard {
            name: name.clone(),
            description: description.into(),
            url: format!("bus://local/{name}"),
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
        self.register(card.clone());
        card
    }

    /// Create a unique ephemeral agent name.
    pub fn ephemeral_name(prefix: &str) -> String {
        let short_id = &Uuid::new_v4().to_string()[..8];
        format!("{prefix}-{short_id}")
    }
}
