//! Server construction and authenticated router assembly.
use super::*;
impl<H: Application> A2AServer<H> {
    /// Create a new A2A server without a bus
    /// # Arguments
    /// `agent_card` is the metadata exposed by this server.
    /// # Returns
    /// A server with an empty shared task table.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>(card: codetether_a2a_core::AgentCard) {
    /// let _server = codetether_a2a_server::A2AServer::<H>::new(card);
    /// # }
    /// ```
    pub fn new(agent_card: AgentCard) -> Self {
        Self {
            application: PhantomData,
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: None,
        }
    }

    /// Create a new A2A server with a bus for event streaming
    /// # Arguments
    /// `agent_card` supplies metadata; `bus` receives server activity.
    /// # Returns
    /// A server sharing the supplied bus and a new task table.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>(card: codetether_a2a_core::AgentCard, bus: std::sync::Arc<codetether_bus_core::AgentBus>) {
    /// let _server = codetether_a2a_server::A2AServer::<H>::with_bus(card, bus);
    /// # }
    /// ```
    pub fn with_bus(agent_card: AgentCard, bus: Arc<codetether_bus_core::AgentBus>) -> Self {
        Self {
            application: PhantomData,
            tasks: Arc::new(DashMap::new()),
            agent_card,
            bus: Some(bus),
        }
    }

    /// Create the router for A2A endpoints.
    ///
    /// RPC calls require a Bearer token when `CODETETHER_AUTH_TOKEN` is
    /// set; agent-card discovery paths stay public.
    /// # Arguments
    /// Consumes the server state used by request handlers.
    /// # Returns
    /// The router with A2A routes, authentication middleware and state attached.
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>(server: codetether_a2a_server::A2AServer<H>) {
    /// let _router = server.router();
    /// # }
    /// ```
    pub fn router(self) -> Router {
        codetether_a2a_http::server::routes::<Self>()
            .layer(axum::middleware::from_fn(
                codetether_a2a_http::auth::require_a2a_auth,
            ))
            .with_state(self)
    }

    /// Get the agent card for this server
    /// # Examples
    /// ```no_run
    /// # fn example<H: codetether_a2a_server::Application>(server: &codetether_a2a_server::A2AServer<H>) {
    /// let _card = server.card();
    /// # }
    /// ```
    #[allow(dead_code)]
    pub fn card(&self) -> &AgentCard {
        &self.agent_card
    }
}
