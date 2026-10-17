//! Attach event, bus, registry, delegation and persistence integrations.
use super::*;
impl RalphLoop {
    /// Attach an event channel for TUI updates
    pub fn with_event_tx(mut self, tx: mpsc::Sender<RalphEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }

    /// Attach an agent bus for inter-iteration learning and context sharing
    pub fn with_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }

    /// Attach a provider registry for relay team planning
    pub fn with_registry(mut self, registry: Arc<ProviderRegistry>) -> Self {
        self.registry = Some(registry);
        self
    }

    /// Attach CADMAS-CTX delegation state for LCB relay provider selection.
    pub fn with_delegation(mut self, state: DelegationState) -> Self {
        self.delegation = Some(state);
        self
    }

    /// Attach a state store for persistent run state
    pub fn with_store(mut self, store: Arc<dyn RalphStateStore>) -> Self {
        self.store = Some(store);
        self
    }

    /// Set a specific run ID (defaults to a generated UUID)
    pub fn with_run_id(mut self, run_id: String) -> Self {
        self.run_id = run_id;
        self
    }
}
