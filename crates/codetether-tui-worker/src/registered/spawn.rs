//! Startup through the registration policy attached to the handle type.
use super::{RegisteredBridge, Registration};
use codetether_bus_core::AgentBus;
use std::sync::Arc;
impl<R: Registration> RegisteredBridge<R> {
    /// Spawn a bridge when server credentials are available.
    /// # Arguments
    /// * `server_url` — Explicit server or application configuration fallback.
    /// * `token` — Explicit token or saved credentials.
    /// * `worker_name` — Explicit worker name or process-derived default.
    /// * `bus` — Shared agent event bus.
    /// # Returns
    /// An owned bridge, or `None` when connection settings are unavailable.
    /// # Errors
    /// Returns configuration loading errors.
    /// # Panics
    /// Panics outside a Tokio runtime if startup requires spawning tasks.
    /// # Examples
    /// ```no_run
    /// # async fn example<R: codetether_tui_worker::Registration>(bus: std::sync::Arc<codetether_bus_core::AgentBus>) -> anyhow::Result<()> {
    /// let _bridge = codetether_tui_worker::RegisteredBridge::<R>::spawn(None, None, None, bus).await?;
    /// # Ok(()) }
    /// ```
    pub async fn spawn(
        server_url: Option<String>,
        token: Option<String>,
        worker_name: Option<String>,
        bus: Arc<AgentBus>,
    ) -> anyhow::Result<Option<Self>> {
        let Some(bridge) = crate::spawn::<R>(server_url, token, worker_name, bus).await? else {
            return Ok(None);
        };
        Ok(Some(bridge.into()))
    }
}
