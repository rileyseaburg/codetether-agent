use super::*;
impl AgentRegistry {
    /// Register an agent card.  Overwrites any previous card for the same id.
    pub fn register(&self, card: AgentCard) {
        let id = card.name.clone();
        tracing::info!(agent_id = %id, "Agent registered on bus");
        self.cards.insert(id, card);
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
}
