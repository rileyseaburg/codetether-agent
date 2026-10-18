//! Bind the native A2A task runtime to startup transport construction.
use crate::a2a;
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use std::sync::Arc;
pub(super) fn build(addr: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
    let agent_card = a2a::server::A2AServer::default_card(&format!("http://{}", addr));
    let a2a_server = a2a::server::A2AServer::with_bus(agent_card.clone(), bus.clone());
    let a2a_router = a2a_server.clone().router();
    let a2a_nested_router = a2a_server.router();
    (agent_card, a2a_router, a2a_nested_router)
}
