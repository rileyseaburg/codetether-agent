//! Adapt the existing application Ralph runtime to the shared pipeline.
use crate::{
    bus::AgentBus,
    provider::{Provider, ProviderRegistry},
    ralph::{RalphConfig, RalphLoop, RalphState},
};
use anyhow::Result;
use async_trait::async_trait;
use std::{path::PathBuf, sync::Arc};
/// Existing application Ralph loop and persistence integration.
pub struct AppHost;
#[async_trait]
impl codetether_go_ralph::Host for AppHost {
    type Registry = ProviderRegistry;
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        bus: Option<Arc<AgentBus>>,
        registry: Option<Arc<ProviderRegistry>>,
    ) -> Result<RalphState> {
        codetether_go_ralph::runtime::run(
            RalphLoop::new(prd_path, provider, model, config),
            bus,
            registry,
        )
        .await
    }
}
