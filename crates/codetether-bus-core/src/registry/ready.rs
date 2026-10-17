use super::*;
impl AgentRegistry {
    /// Register an agent from a protocol-level ready announcement.
    ///
    /// This normalizes the ready payload into an `AgentCard` so all components
    /// (TUI, worker, swarm) can rely on the bus registry as source of truth.
    pub fn register_ready(&self, agent_id: &str, capabilities: &[String]) -> AgentCard {
        let skills = super::skills::build(capabilities);
        let card = super::card::build(agent_id, skills);
        self.register(card.clone());
        card
    }
}
