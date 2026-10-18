//! Application A2A and HTTP routing for native server startup.
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use std::sync::Arc;

pub(super) type AppHost = codetether_native_server::Host<
    crate::tool::registry_lifecycle::Native,
    Routing,
    { cfg!(test) },
>;
pub(super) struct Routing;
impl codetether_native_server::Routes<crate::tool::registry_lifecycle::Native, { cfg!(test) }>
    for Routing
{
    fn a2a(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
        crate::a2a::server::A2AServer::startup_routers(address, bus)
    }
    fn router(state: &super::AppState, root: Router, nested: Router) -> Router {
        super::http_router::build(state, root, nested)
    }
}
