//! Bind process identity to the shared agent-card extension.
use crate::a2a::types::AgentCard;
use codetether_a2a_core::identity_extension::attach_with_claims;
pub(crate) use codetether_a2a_core::identity_extension::from_card;

/// Adds the routable provenance identity extension to an agent card.
pub(crate) fn attach(card: &mut AgentCard, identity: &str) {
    let persona_id = crate::provenance::runtime_persona_id();
    let spiffe_id = crate::provenance::runtime_spiffe_id();
    attach_with_claims(card, identity, persona_id.as_deref(), spiffe_id.as_deref());
}

#[cfg(test)]
#[path = "agent_identity_tests.rs"]
mod tests;
