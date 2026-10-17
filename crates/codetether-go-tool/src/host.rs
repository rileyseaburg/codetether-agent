//! Application bindings for provider discovery and Ralph execution.

use crate::{Completion, Registry};
use anyhow::Result;
use async_trait::async_trait;
use codetether_bus_core::AgentBus;
use codetether_okr::{Okr, OkrRun};
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Application integration needed by the autonomous task tool.
///
/// # Implementors
/// Preserve provider discovery, pipeline execution and result formatting semantics.
#[async_trait]
pub trait Host: Send + Sync + 'static {
    /// Existing provider registry implementation.
    type Registry: Registry;
    /// Native pipeline result before rendering.
    type Output: Send + 'static;
    /// Load the shared provider registry.
    ///
    /// # Errors
    /// Returns the application's provider-discovery error.
    async fn registry() -> Result<Arc<Self::Registry>>;
    /// Execute the existing Ralph pipeline with the supplied context.
    ///
    /// # Errors
    /// Returns any pipeline error without converting it to success.
    #[allow(clippy::too_many_arguments)]
    async fn execute(
        task: &str,
        okr: &mut Okr,
        run: &mut OkrRun,
        provider: Arc<dyn Provider>,
        model: &str,
        max_iterations: usize,
        bus: Option<Arc<AgentBus>>,
        max_concurrent: usize,
        registry: Option<Arc<Self::Registry>>,
    ) -> Result<Self::Output>;
    /// Render the native result after the updated run has been persisted.
    fn summarize(result: Self::Output, task: &str) -> Completion;
}
