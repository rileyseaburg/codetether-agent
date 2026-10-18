//! Mutable state of one relay execution.
use crate::Host;
use crate::team::Team;
use codetether_autochat_core::shared_context::SharedRelayContext;
use codetether_bus_core::{AgentBus, BusHandle, relay::ProtocolRelayRuntime};
use std::{collections::HashMap, sync::Arc};
pub(super) struct Runtime<H: Host> {
    pub bus: Arc<AgentBus>,
    pub relay: ProtocolRelayRuntime,
    pub registry: Option<Arc<H::Registry>>,
    pub team: Team<H::Session>,
    pub context_receiver: BusHandle,
    pub shared_context: SharedRelayContext,
    pub kr_targets: HashMap<String, f64>,
    pub kr_progress: HashMap<String, f64>,
    pub baton: String,
    pub previous_normalized: Option<String>,
    pub convergence_hits: usize,
    pub turns: usize,
    pub dynamic_spawn_count: usize,
    pub rlm_handoff_count: usize,
    pub rlm_context_count: usize,
    pub status: String,
    pub failure_note: Option<String>,
    pub planner_used: bool,
}
