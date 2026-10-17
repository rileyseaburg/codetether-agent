use super::*;
impl Orchestrator {
    /// Get statistics
    pub fn stats(&self) -> &SwarmStats {
        self.state.stats()
    }

    /// Get mutable statistics
    pub fn stats_mut(&mut self) -> &mut SwarmStats {
        self.state.stats_mut()
    }

    /// Get the provider registry
    pub fn providers(&self) -> &ProviderRegistry {
        &self.providers
    }

    /// Get current model
    pub fn model(&self) -> &str {
        &self.model
    }

    /// Get current provider
    pub fn provider(&self) -> &str {
        &self.provider
    }
}
