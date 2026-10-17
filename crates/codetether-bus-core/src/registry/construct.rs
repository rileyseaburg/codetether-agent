use super::*;
impl AgentRegistry {
    /// Create an empty registry.
    pub fn new() -> Self {
        Self {
            cards: DashMap::new(),
        }
    }
}

impl Default for AgentRegistry {
    fn default() -> Self {
        Self::new()
    }
}
