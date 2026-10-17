//! Adapt the existing application Ralph runtime to the shared pipeline.
use crate::{
    bus::AgentBus,
    provider::{Provider, ProviderRegistry},
    ralph::{RalphConfig, RalphLoop, RalphState, store_http::HttpStore},
};
use anyhow::{Context, Result};
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
        let mut ralph = RalphLoop::new(prd_path, provider, model, config)
            .await
            .context("Failed to initialize Ralph loop")?;

        // Attach bus for inter-iteration learning sharing
        if let Some(bus) = bus {
            ralph = ralph.with_bus(bus);
        }

        // Attach registry for relay team planning
        if let Some(registry) = registry {
            ralph = ralph.with_registry(registry);
        }

        // Attach state store for persistent run tracking
        ralph = ralph.with_store(Arc::new(HttpStore::from_env()));

        ralph.run().await.context("Ralph loop execution failed")
    }
}
