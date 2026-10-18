//! Bind the autonomous pipeline to the application Ralph host.
use super::{GoRalphResult, host::AppHost};
use crate::{
    bus::AgentBus,
    okr::{Okr, OkrRun},
    provider::{Provider, ProviderRegistry},
};
use anyhow::Result;
use std::sync::Arc;
/// Run the approved task through the application's existing Ralph loop.
///
/// # Arguments
/// Arguments retain the parent pipeline's task, objective, provider and limits.
///
/// # Returns
/// The completed pipeline summary.
///
/// # Errors
/// Forwards generation, persistence and Ralph runtime errors.
///
/// # Panics
/// Preserves PRD generation's byte-based repair excerpt behavior.
///
/// # Examples
/// See [codetether_go_ralph::execute_go_ralph] for the shared pipeline contract.
#[allow(clippy::too_many_arguments)]
pub async fn execute_go_ralph(
    task: &str,
    okr: &mut Okr,
    okr_run: &mut OkrRun,
    provider: Arc<dyn Provider>,
    model: &str,
    max_iterations: usize,
    bus: Option<Arc<AgentBus>>,
    max_concurrent_stories: usize,
    registry: Option<Arc<ProviderRegistry>>,
) -> Result<GoRalphResult> {
    codetether_go_ralph::execute_go_ralph::<AppHost>(
        task,
        okr,
        okr_run,
        provider,
        model,
        max_iterations,
        bus,
        max_concurrent_stories,
        registry,
    )
    .await
}
