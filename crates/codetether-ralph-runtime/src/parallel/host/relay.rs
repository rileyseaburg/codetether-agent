//! Forward the borrowed task inputs to the existing relay runtime.
use super::*;
pub(super) async fn run<H: Host>(
    context: &Context<'_, TaskHost<H>>,
    registry: &Arc<H::Registry>,
    prompt: &str,
) -> anyhow::Result<String> {
    RalphLoop::<H>::call_relay_static(
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
