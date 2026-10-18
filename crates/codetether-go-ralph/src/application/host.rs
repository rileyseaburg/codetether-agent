//! Construct the shared Ralph loop and attach existing pipeline services.
use super::ApplicationHost;
use crate::Host;
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use codetether_ralph_runtime::{Host as RuntimeHost, RalphLoop};
use std::{path::PathBuf, sync::Arc};
#[async_trait::async_trait]
impl<P, H> Host for ApplicationHost<P, RalphLoop<H>>
where
    P: codetether_go_prd::Registry + 'static,
    H: RuntimeHost<Registry = P>,
{
    type Registry = P;
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        bus: Option<Arc<AgentBus>>,
        registry: Option<Arc<P>>,
    ) -> Result<RalphState> {
        crate::runtime::run(
            RalphLoop::<H>::new(prd_path, provider, model, config),
            bus,
            registry,
        )
        .await
    }
}
