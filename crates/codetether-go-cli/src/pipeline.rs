//! Execute the approved objective and print the final CLI result.
use crate::{Host, Input};
use codetether_autochat_core::model_rotation::{
    ModelRegistry, resolve_provider_for_model_autochat,
};
use codetether_okr::{Okr, OkrRepository, OkrRun};
use std::sync::Arc;
pub(super) async fn run<H: Host>(
    input: Input,
    max_concurrent: usize,
    mut okr: Okr,
    mut approved_run: OkrRun,
    registry_for_draft: Option<Arc<H::Registry>>,
) -> anyhow::Result<()>
where
    H::Registry: ModelRegistry,
{
    let task = &input.task;
    let model = &input.model;
    let registry = if let Some(registry) = registry_for_draft {
        registry
    } else {
        H::load_registry().await?
    };
    let (provider, resolved_model) = resolve_provider_for_model_autochat(registry.as_ref(), model)
        .ok_or_else(|| anyhow::anyhow!("No provider available for model '{model}'"))?;

    // Wire bus for training data capture
    let bus = H::create_bus();

    // Execute via Ralph PRD loop — use max_concurrent as concurrency
    let ralph_result = codetether_go_ralph::execute_go_ralph::<H>(
        task,
        &mut okr,
        &mut approved_run,
        provider,
        &resolved_model,
        10,                     // max iterations
        Some(bus),              // bus for training data
        max_concurrent,         // max concurrent stories
        Some(registry.clone()), // relay registry
    )
    .await?;

    // Persist final run state
    if let Ok(repo) = OkrRepository::from_config().await {
        let _ = repo.update_run(approved_run).await;
    }

    // Display results
    codetether_run_report::render_ralph(&input.format, &ralph_result, task)?;

    Ok(())
}
