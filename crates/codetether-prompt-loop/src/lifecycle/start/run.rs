use super::Host;
use codetether_session_events::SessionEvent;
/// Prepare and announce one provider step in the established order.
/// # Arguments
/// `host` supplies native services; `step` identifies the step for logging.
/// # Returns
/// Success after cost-budget enforcement.
/// # Errors
/// Propagates execution-boundary, model-preparation and cost-budget failures.
/// # Examples
/// ```no_run
/// # async fn begin(host: &mut impl codetether_prompt_loop::lifecycle::start::Host) -> anyhow::Result<()> {
/// codetether_prompt_loop::lifecycle::start::run(host, 1).await?;
/// # Ok(()) }
/// ```
pub async fn run(host: &mut impl Host, step: usize) -> anyhow::Result<()> {
    host.ensure_running().await?;
    host.drain_steering();
    host.prepare()?;
    tracing::info!(step, "Agent step starting");
    host.inject_notices();
    if let Some(tx) = host.events() {
        let _ = tx.send(SessionEvent::Thinking).await;
    }
    host.enforce_cost()
}
