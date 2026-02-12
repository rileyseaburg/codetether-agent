//! Agent registry — tracks connected agents and their cards.
//!
//! Every agent that joins the bus registers an `AgentCard`, which is stored
//! in a concurrent `DashMap`.  The registry also provides an ephemeral card
//! factory for sub-agents that need a short-lived identity.

use crate::a2a::types::{AgentCapabilities, AgentCard, AgentSkill};
use dashmap::DashMap;
use uuid::Uuid;

/// Thread-safe registry of agent cards keyed by agent id.
pub struct AgentRegistry {
    cards: DashMap<String, AgentCard>,
}

impl AgentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            cards: DashMap::new(),
        }
    }

    /// Register an agent card.  Overwrites any previous card for the same id.
    pub fn register(&self, card: AgentCard) {
        let id = card.name.clone();
        tracing::info!(agent_id = %id, "Agent registered on bus");
        self.cards.insert(id, card);
    }

    /// Register an agent from a protocol-level ready announcement.
    ///
    /// This normalizes the ready payload into an `AgentCard` so all components
    /// (TUI, worker, swarm) can rely on the bus registry as source of truth.
    pub fn register_ready(&self, agent_id: &str, capabilities: &[String]) -> AgentCard {
        let skills = capabilities
            .iter()
            .map(|capability| {
                let id = sanitize_skill_id(capability);
                AgentSkill {
                    id,
                    name: capability.clone(),
                    description: format!("Capability: {capability}"),
                    tags: vec!["protocol".to_string(), "bus".to_string()],
                    examples: vec![],
                    input_modes: vec!["text".to_string()],
                    output_modes: vec!["text".to_string()],
                }
            })
            .collect::<Vec<_>>();

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

        self.register(card.clone());
        card
    }

    /// Deregister an agent by id.
    pub fn deregister(&self, agent_id: &str) -> Option<AgentCard> {
        tracing::info!(agent_id = %agent_id, "Agent deregistered from bus");
        self.cards.remove(agent_id).map(|(_, card)| card)
    }

    /// Look up a card by agent id.
    pub fn get(&self, agent_id: &str) -> Option<AgentCard> {
        self.cards.get(agent_id).map(|r| r.value().clone())
    }

    /// List all registered agent ids.
    pub fn agent_ids(&self) -> Vec<String> {
        self.cards.iter().map(|r| r.key().clone()).collect()
    }

    /// Number of registered agents.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Whether the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

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

fn sanitize_skill_id(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    for ch in raw.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
        } else if !out.ends_with('-') {
            out.push('-');
        }
    }

    let trimmed = out.trim_matches('-');
    if trimmed.is_empty() {
        "capability".to_string()
    } else {
        trimmed.to_string()
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_lookup() {
        let reg = AgentRegistry::new();
        let card = reg.create_ephemeral("agent-1", "Test agent", vec![]);
        assert_eq!(reg.len(), 1);

        let found = reg.get("agent-1").unwrap();
        assert_eq!(found.url, card.url);
    }

    #[test]
    fn test_deregister() {
        let reg = AgentRegistry::new();
        reg.create_ephemeral("agent-2", "temp", vec![]);
        assert_eq!(reg.len(), 1);

        let removed = reg.deregister("agent-2");
        assert!(removed.is_some());
        assert_eq!(reg.len(), 0);
    }

    #[test]
    fn test_ephemeral_name_unique() {
        let a = AgentRegistry::ephemeral_name("sub");
        let b = AgentRegistry::ephemeral_name("sub");
        assert_ne!(a, b);
    }

    #[test]
    fn test_agent_ids() {
        let reg = AgentRegistry::new();
        reg.create_ephemeral("alpha", "a", vec![]);
        reg.create_ephemeral("beta", "b", vec![]);
        let mut ids = reg.agent_ids();
        ids.sort();
        assert_eq!(ids, vec!["alpha", "beta"]);
    }

    #[test]
    fn test_register_ready_creates_card_with_skills() {
        let reg = AgentRegistry::new();
        let caps = vec!["plan.tasks".to_string(), "tool.call".to_string()];
        let card = reg.register_ready("agent-ready-1", &caps);

        assert_eq!(card.name, "agent-ready-1");
        assert_eq!(card.url, "bus://local/agent-ready-1");
        assert_eq!(card.skills.len(), 2);
        assert!(reg.get("agent-ready-1").is_some());
    }
}
