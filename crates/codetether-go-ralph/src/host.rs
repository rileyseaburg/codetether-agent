//! Application-owned Ralph-loop execution boundary.

use anyhow::Result;
use async_trait::async_trait;
use codetether_bus_core::AgentBus;
use codetether_provider_core::Provider;
use codetether_ralph_core::{RalphConfig, RalphState};
use std::{path::PathBuf, sync::Arc};
/// Run the application's configured Ralph loop with shared contracts.
///
/// # Implementors
/// Preserve loop construction, bus and registry attachment, state-store setup,
/// and error context in the application's runtime adapter.
#[async_trait]
pub trait Host: Send + Sync + 'static {
    /// Existing provider registry used for PRD generation and relay planning.
    type Registry: codetether_go_prd::Registry + 'static;
    /// Execute the Ralph loop and return its terminal state.
    ///
    /// # Errors
    /// Returns loop initialization or execution errors with their original context.
    async fn run(
        prd_path: PathBuf,
        provider: Arc<dyn Provider>,
        model: String,
        config: RalphConfig,
        bus: Option<Arc<AgentBus>>,
        registry: Option<Arc<Self::Registry>>,
    ) -> Result<RalphState>;
}
