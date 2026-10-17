//! Coordinate PRD generation, audit, Ralph execution and objective outcomes.

use crate::{GoRalphResult, Host};
use anyhow::Result;
use codetether_bus_core::AgentBus;
use codetether_go_prd::Registry;
use codetether_okr::{Okr, OkrRun};
use codetether_provider_core::Provider;
use std::sync::Arc;
/// Execute the approved task pipeline using an application Ralph host.
///
/// # Arguments
/// * `task`, `okr`, `okr_run` — Description and mutable objective/run records.
/// * `provider`, `model` — Configured primary completion provider and model.
/// * `max_iterations`, `max_concurrent_stories` — Existing execution limits.
/// * `bus`, `registry` — Optional learning bus and configured providers.
///
/// # Returns
/// Terminal story summary after objective updates and completion audit.
///
/// # Errors
/// Returns generation, PRD persistence, initialization or execution errors.
///
/// # Panics
/// Preserves PRD generation's byte-based repair excerpt behavior.
///
/// # Examples
/// Implement [Host] to supply the application's existing Ralph loop.
#[allow(clippy::too_many_arguments)]
pub async fn execute_go_ralph<H: Host>(
    task: &str,
    okr: &mut Okr,
    okr_run: &mut OkrRun,
    provider: Arc<dyn Provider>,
    model: &str,
    max_iterations: usize,
    bus: Option<Arc<AgentBus>>,
    max_concurrent_stories: usize,
    registry: Option<Arc<H::Registry>>,
) -> Result<GoRalphResult> {
    // Step 1: Generate PRD from task + KRs (with automatic model retry)
    tracing::info!(target: "codetether_agent::cli::go_ralph",task = %task, okr_id = %okr.id, "Generating PRD from task and key results");
    let registry_ref = registry.as_ref().map(|r| r.as_ref() as &dyn Registry);
    let prd = codetether_go_prd::generate_prd_with_model_retry(
        task,
        okr,
        Arc::clone(&provider),
        model,
        registry_ref,
    )
    .await?;

    let (prd_filename, prd_path) = crate::persist::save(&prd, okr_run).await?;
    crate::audit_generated::record(task, okr, okr_run, &prd, &prd_path).await;
    crate::start::mark(okr_run, &prd_filename);
    let config = crate::config::build(
        &prd_path,
        okr_run,
        model,
        max_iterations,
        max_concurrent_stories,
    );
    let state = H::run(
        prd_path.clone(),
        Arc::clone(&provider),
        model.to_string(),
        config,
        bus,
        registry,
    )
    .await?;
    crate::finish::complete(okr, okr_run, state, prd_path).await
}
