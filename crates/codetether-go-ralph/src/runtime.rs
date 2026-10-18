//! Ralph loop initialization, service attachment, and execution.
use anyhow::{Context, Result};
use codetether_bus_core::AgentBus;
use codetether_ralph_core::{RalphState, store_http::HttpStore};
use codetether_ralph_runtime::{Host, RalphLoop};
use std::{future::Future, sync::Arc};

/// Initialize a Ralph loop, attach its services, and execute it.
/// # Arguments
/// * `initialize` — Application-specialized loop constructor future.
/// * `bus` — Optional bus used for inter-iteration learning.
/// * `registry` — Optional provider registry used for relay planning.
/// # Returns
/// Terminal Ralph state after running with HTTP state persistence configured
/// from the environment.
/// # Errors
/// Returns initialization or execution errors with their original context.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_ralph_runtime::Host>(provider: std::sync::Arc<dyn codetether_provider_core::Provider>) -> anyhow::Result<()> {
/// let initialize = codetether_ralph_runtime::RalphLoop::<H>::new(
///     "prd.json".into(), provider, "model".into(), Default::default(),
/// );
/// let _state = codetether_go_ralph::runtime::run(initialize, None, None).await?;
/// # Ok(()) }
/// ```
pub async fn run<H: Host>(
    initialize: impl Future<Output = Result<RalphLoop<H>>>,
    bus: Option<Arc<AgentBus>>,
    registry: Option<Arc<H::Registry>>,
) -> Result<RalphState> {
    let mut ralph = initialize
        .await
        .context("Failed to initialize Ralph loop")?;
    if let Some(bus) = bus {
        ralph = ralph.with_bus(bus);
    }
    if let Some(registry) = registry {
        ralph = ralph.with_registry(registry);
    }
    ralph = ralph.with_store(Arc::new(HttpStore::from_env()));
    ralph.run().await.context("Ralph loop execution failed")
}
