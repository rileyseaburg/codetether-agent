//! Drive the background pipeline and dispatch its terminal outcome.

use crate::Host;
use codetether_go_monitor::{ACTIVE_GO_RUNS, GoRunPhase};
use codetether_okr::{Okr, OkrRun};
use codetether_provider_core::Provider;
use std::sync::Arc;
#[allow(clippy::too_many_arguments)]
pub(crate) async fn run<H: Host>(
    mut okr: Okr,
    mut run: OkrRun,
    provider: Arc<dyn Provider>,
    registry: Arc<H::Registry>,
    bg_okr_id: String,
    bg_task: String,
    bg_model: String,
    max_iterations: usize,
    max_concurrent: usize,
    bg_callback: Option<Arc<dyn Fn(String) + Send + Sync>>,
) {
    // Update phase to Running
    if let Ok(mut runs) = ACTIVE_GO_RUNS.lock()
        && let Some(r) = runs.get_mut(&bg_okr_id)
    {
        r.phase = GoRunPhase::Running;
    }

    // Create bus with S3 sink for training data archival
    let bus = codetether_bus_core::AgentBus::new().into_arc();
    codetether_bus_s3::spawn_bus_s3_sink(bus.clone());

    match H::execute(
        &bg_task,
        &mut okr,
        &mut run,
        provider,
        &bg_model,
        max_iterations,
        Some(bus),
        max_concurrent,
        Some(registry),
    )
    .await
    {
        Ok(result) => {
            crate::success::complete::<H>(run, result, bg_task, bg_okr_id, bg_callback).await
        }
        Err(err) => crate::failure::record(run, err, bg_okr_id, bg_callback).await,
    }
}
