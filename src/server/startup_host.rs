//! Native runtime adapters for the shared server startup sequence.
mod a2a;
mod state;
mod tools;
use crate::cognition::CognitionRuntime;
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use codetether_plugin_verification::PluginRegistry;
use codetether_server_runtime::Parts;
use std::sync::Arc;

pub(super) struct AppHost;
impl codetether_server_runtime::Host for AppHost {
    type Runtime = CognitionRuntime;
    type State = super::AppState;
    fn new_runtime() -> CognitionRuntime {
        CognitionRuntime::new_from_env()
    }
    fn install_tools(runtime: &mut CognitionRuntime) -> PluginRegistry {
        tools::install(runtime)
    }
    fn runtime_enabled(runtime: &CognitionRuntime) -> bool {
        runtime.is_enabled()
    }
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
        a2a::build(address, bus)
    }
    fn state(parts: Parts<CognitionRuntime>) -> super::AppState {
        state::build(parts)
    }
    fn router(state: &super::AppState, root: Router, nested: Router) -> Router {
        super::http_router::build(state, root, nested)
    }
}
