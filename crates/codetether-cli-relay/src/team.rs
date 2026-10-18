//! Ordered sessions, handoff receivers and model rotation.
use codetether_autochat_core::model_rotation::RelayModelRotation;
use codetether_bus_core::BusHandle;
use std::collections::HashMap;
pub(super) struct Team<S> {
    pub ordered_agents: Vec<String>,
    pub sessions: HashMap<String, S>,
    pub relay_receivers: HashMap<String, BusHandle>,
    pub agent_models: HashMap<String, String>,
    pub model_rotation: RelayModelRotation,
}
