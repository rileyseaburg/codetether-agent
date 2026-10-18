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
        codetether_native_server::build_router::<
            crate::tool::registry_lifecycle::Native,
            crate::session::Session,
            super::session_realtime::AppHost,
            { cfg!(test) },
        >(
            state,
            root,
            nested,
            env!("CARGO_PKG_VERSION"),
            env!("CARGO_PKG_NAME"),
        )
    }
}
