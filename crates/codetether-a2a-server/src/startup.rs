//! Build the paired startup routers from one shared A2A task runtime.
use crate::{A2AServer, Application};
use axum::Router;
use codetether_a2a_core::AgentCard;
use codetether_bus_core::AgentBus;
use std::sync::Arc;
impl<H: Application> A2AServer<H> {
    /// Build the default card and root/nested routers for an HTTP bind address.
    /// # Arguments
    /// `address` is the host/port; `bus` is shared by both routers.
    /// # Returns
    /// The advertised card, root router and nested router in startup order.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>(bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
    /// let (_card, _root, _nested) = codetether_a2a_server::A2AServer::<H>::startup_routers("127.0.0.1:8000", bus);
    /// # }
    /// ```
    pub fn startup_routers(address: &str, bus: Arc<AgentBus>) -> (AgentCard, Router, Router) {
        let agent_card = Self::default_card(&format!("http://{address}"));
        let a2a_server = Self::with_bus(agent_card.clone(), bus.clone());
        let a2a_router = a2a_server.clone().router();
        let a2a_nested_router = a2a_server.router();
        (agent_card, a2a_router, a2a_nested_router)
    }
}
