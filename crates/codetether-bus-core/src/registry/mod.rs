//! Concurrent agent cards and ready/shutdown registration policy.
//!
//! [AgentRegistry] stores cards by agent ID and builds ephemeral identities.
//! [apply] updates registration for shared bus lifecycle messages.
use codetether_a2a_core::{AgentCapabilities, AgentCard, AgentSkill};
use dashmap::DashMap;
use uuid::Uuid;
mod access;
mod card;
mod construct;
mod effects;
mod ephemeral;
mod ready;
mod skill_id;
mod skills;
pub use effects::apply;
#[cfg(test)]
mod tests;

/// Thread-safe registry of agent cards keyed by agent id.
///
/// # Examples
/// ```
/// let registry = codetether_bus_core::registry::AgentRegistry::new();
/// let card = registry.register_ready("agent", &["read".into()]);
/// assert_eq!(registry.get("agent").unwrap().name, card.name);
/// ```
pub struct AgentRegistry {
    cards: DashMap<String, AgentCard>,
}
