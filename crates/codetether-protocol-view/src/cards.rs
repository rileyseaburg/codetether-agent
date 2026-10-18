//! Build ordered display cards from registered agent names.
use crate::RegistryView;
use codetether_a2a_core::AgentCard;

pub(crate) fn collect_agent_cards(view: &RegistryView<'_>) -> Vec<AgentCard> {
    let mut names = view.registered_agents.iter().cloned().collect::<Vec<_>>();
    names.sort();
    names
        .into_iter()
        .map(|name| agent_card(view, name))
        .collect()
}

pub(crate) fn agent_card(view: &RegistryView<'_>, name: String) -> AgentCard {
    AgentCard {
        name,
        description: String::new(),
        url: view
            .worker_id
            .as_ref()
            .cloned()
            .unwrap_or_else(|| "unknown".into()),
        version: "0.3.0".into(),
        protocol_version: "0.3.0".into(),
        preferred_transport: None,
        additional_interfaces: vec![],
        capabilities: Default::default(),
        skills: vec![],
        default_input_modes: vec![],
        default_output_modes: vec![],
        provider: None,
        icon_url: None,
        documentation_url: None,
        security_schemes: Default::default(),
        security: vec![],
        supports_authenticated_extended_card: false,
        signatures: vec![],
    }
}
