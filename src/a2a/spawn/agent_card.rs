//! Agent-card construction and runtime identity binding.

use crate::a2a::{agent_identity, server::A2AServer, types::AgentCard};

pub(super) fn build(name: &str, url: &str, description: Option<&str>) -> AgentCard {
    let identity = crate::provenance::ensure_runtime_agent_identity(name);
    let mut card = A2AServer::default_card(url);
    card.name = name.to_string();
    card.description = super::card_description::resolve(description);
    agent_identity::attach(&mut card, &identity);
    card
}
