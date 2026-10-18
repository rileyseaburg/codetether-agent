//! Initialized services awaiting relay-state assembly.
use crate::{Host, team::Team};
use codetether_autochat_core::shared_context::SharedRelayContext;
use codetether_bus_core::{AgentBus, BusHandle, relay::ProtocolRelayRuntime};
use std::{collections::HashMap, sync::Arc};
pub(super) struct Initial<H: Host> {
    pub bus: Arc<AgentBus>,
    pub relay: ProtocolRelayRuntime,
    pub registry: Option<Arc<H::Registry>>,
    pub team: Team<H::Session>,
    pub context_receiver: BusHandle,
    pub shared_context: SharedRelayContext,
    pub kr_targets: HashMap<String, f64>,
    pub planner_used: bool,
}
