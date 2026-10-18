//! CLI services on the existing application Ralph host.
use super::host::AppHost;
use crate::{bus::AgentBus, provider::ProviderRegistry};
use std::sync::Arc;
#[async_trait::async_trait(?Send)]
impl codetether_go_cli::Host for AppHost {
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry>> {
        ProviderRegistry::shared_from_vault().await
    }
    fn create_bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());
        bus
    }
}
