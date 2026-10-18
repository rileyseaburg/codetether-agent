//! Forward the borrowed task inputs to the existing relay runtime.
use super::*;
pub(super) async fn run(
    context: &Context<'_, Native>,
    registry: &Arc<ProviderRegistry>,
    prompt: &str,
) -> anyhow::Result<String> {
    RalphLoop::call_relay_static(
        registry,
        context.model,
        prompt,
        context.working_dir,
        context.ralph_tx.clone(),
        context.story.id.clone(),
        context.bus.clone(),
        context.relay_max_agents,
        context.relay_max_rounds,
        context.delegation,
    )
    .await
}
