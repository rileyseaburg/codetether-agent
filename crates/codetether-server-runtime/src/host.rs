//! Native runtime and router construction boundaries for startup.
use crate::Parts;
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use codetether_cognition_api::http::Lifecycle;
use codetether_plugin_verification::PluginRegistry;
use std::sync::Arc;
/// Native bindings required by the shared startup sequence.
/// # Implementors
/// Applications supply runtime tools, A2A routing and concrete request state.
pub trait Host {
    /// Cognition runtime supporting startup and status operations.
    type Runtime: Lifecycle + 'static;
    /// Concrete request state consumed by the application's router adapter.
    type State;
    /// Construct the native cognition runtime from existing environment policy.
    fn new_runtime() -> Self::Runtime;
    /// Install native tools and return their existing plugin registry.
    fn install_tools(runtime: &mut Self::Runtime) -> PluginRegistry;
    /// Return the runtime's configured enabled flag.
    fn runtime_enabled(runtime: &Self::Runtime) -> bool;
    /// Construct the existing A2A card and both bound HTTP routers.
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router);
    /// Assemble native state from initialized services without changing them.
    fn state(parts: Parts<Self::Runtime>) -> Self::State;
    /// Build the application router over its native state and A2A routers.
    fn router(state: &Self::State, root: Router, nested: Router) -> Router;
}
