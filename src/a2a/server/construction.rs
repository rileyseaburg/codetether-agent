//! Server construction and authenticated router assembly.
use super::*;
impl A2AServer {
    /// Create a new A2A server without a bus
    pub fn new(agent_card: AgentCard) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: None,
        }
    }

    /// Create a new A2A server with a bus for event streaming
    pub fn with_bus(agent_card: AgentCard, bus: Arc<crate::bus::AgentBus>) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: Some(bus),
        }
    }

    /// Create the router for A2A endpoints.
    ///
    /// RPC calls require a Bearer token when `CODETETHER_AUTH_TOKEN` is
    /// set; agent-card discovery paths stay public.
    pub fn router(self) -> Router {
        codetether_a2a_http::server::routes::<Self>()
            .layer(axum::middleware::from_fn(
                crate::a2a::server_auth::require_a2a_auth,
            ))
            .with_state(self)
    }

    /// Get the agent card for this server
    #[allow(dead_code)]
    pub fn card(&self) -> &AgentCard {
        &self.agent_card
    }
}
