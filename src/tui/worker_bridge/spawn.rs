//! Adapt shared bridge handles to the native public handle type.
use super::{TuiWorkerBridge, registration::NativeRegistration};
use crate::bus::AgentBus;
use std::sync::Arc;
impl TuiWorkerBridge {
    /// Spawn the bridge when server credentials are available.
    /// # Errors
    /// Returns configuration loading errors.
    pub async fn spawn(
        server_url: Option<String>,
        token: Option<String>,
        worker_name: Option<String>,
        bus: Arc<AgentBus>,
    ) -> anyhow::Result<Option<Self>> {
        let Some(bridge) =
            codetether_tui_worker::spawn::<NativeRegistration>(server_url, token, worker_name, bus)
                .await?
        else {
            return Ok(None);
        };
        Ok(Some(bridge.into()))
    }
}
