//! Bind process provenance to the shared agent-card identity extension.
//!
//! [`attach`] reads the current persona and SPIFFE claims and records them with
//! a caller-supplied routable identity. [`from_card`] reads that identity back.
pub use codetether_a2a_core::identity_extension::from_card;
use codetether_a2a_core::{AgentCard, identity_extension::attach_with_claims};
/// Attach the routable identity and current process claims to an agent card.
/// # Arguments
/// `card` receives the extension; `identity` identifies the routable agent.
/// # Returns
/// Completes after appending the existing identity extension.
/// # Examples
/// ```
/// let mut card = codetether_a2a_core::application_card::default_card("http://localhost:8000", "test");
/// codetether_a2a_http::identity::attach(&mut card, "agent-1");
/// assert_eq!(codetether_a2a_http::identity::from_card(&card).as_deref(), Some("agent-1"));
/// ```
pub fn attach(card: &mut AgentCard, identity: &str) {
    let persona_id = codetether_provenance::runtime_persona_id();
    let spiffe_id = codetether_provenance::runtime_spiffe_id();
    attach_with_claims(card, identity, persona_id.as_deref(), spiffe_id.as_deref());
}
#[cfg(test)]
mod tests;
