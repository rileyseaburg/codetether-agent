//! Registry and training-bus setup for the shared application Ralph host.
use crate::Host;
use codetether_application_providers::{Application, ProviderRegistry};
use codetether_bus_core::AgentBus;
use codetether_go_ralph::application::ApplicationHost;
use codetether_ralph_runtime::{Host as RuntimeHost, RalphLoop};
use std::sync::Arc;
#[async_trait::async_trait(?Send)]
impl<P, H> Host for ApplicationHost<ProviderRegistry<P>, RalphLoop<H>>
where
    P: Application,
    H: RuntimeHost<Registry = ProviderRegistry<P>>,
{
    async fn load_registry() -> anyhow::Result<Arc<ProviderRegistry<P>>> {
        ProviderRegistry::shared_from_vault().await
    }
    fn create_bus() -> Arc<AgentBus> {
        let bus = AgentBus::new().into_arc();
        codetether_bus_s3::spawn_bus_s3_sink(bus.clone());
        bus
    }
}
