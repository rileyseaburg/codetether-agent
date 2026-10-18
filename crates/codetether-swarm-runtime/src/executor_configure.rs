//! Service attachment for shared executor state.
use crate::ExecutorState;
use codetether_bus_core::AgentBus;
use codetether_context_policy::delegation::DelegationState;
use codetether_session_control::ControlHandle;
use codetether_swarm_core::SwarmEvent;
use codetether_telemetry::SwarmTelemetryCollector;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;

impl<A, C> ExecutorState<A, C> {
    /// Attach the supplied cache to this executor state.
    pub fn with_cache_instance(mut self, cache: Arc<tokio::sync::Mutex<C>>) -> Self {
        self.cache = Some(cache);
        self
    }
    /// Attach the supplied bus to this executor state.
    pub fn with_bus(mut self, bus: Arc<AgentBus>) -> Self {
        self.bus = Some(bus);
        self
    }
    /// Attach the supplied control to this executor state.
    pub fn with_control(mut self, control: ControlHandle) -> Self {
        self.control = Some(control);
        self
    }
    /// Attach the supplied state to this executor state.
    pub fn with_delegation(mut self, state: DelegationState) -> Self {
        self.delegation = Some(Arc::new(Mutex::new(state)));
        self
    }
    /// Attach the supplied agent to this executor state.
    pub fn with_coordinator_agent(mut self, agent: Arc<tokio::sync::Mutex<A>>) -> Self {
        self.coordinator_agent = Some(agent);
        self
    }
    /// Attach the supplied tx to this executor state.
    pub fn with_event_tx(mut self, tx: mpsc::Sender<SwarmEvent>) -> Self {
        self.event_tx = Some(tx);
        self
    }
    /// Attach the supplied telemetry to this executor state.
    pub fn with_telemetry(mut self, telemetry: Arc<SwarmTelemetryCollector>) -> Self {
        self.telemetry = telemetry;
        self
    }
}
