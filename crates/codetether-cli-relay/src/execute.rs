//! Public entry point for the staged relay runtime.
use anyhow::Result;
use codetether_relay_report::AutochatCliResult;
use uuid::Uuid;
/// Run a bounded multi-agent relay and report its final outcome.
/// # Arguments
/// * `agent_count` - Requested initial team size.
/// * `task` - User task to execute.
/// * `model_ref` - Requested provider/model reference.
/// * `okr_id` - Optional objective whose targets will track progress.
/// * `okr_run_id` - Optional run receiving progress and final evidence.
/// # Returns
/// The final status, team, handoff and summary.
/// # Errors
/// Returns initial session creation errors or an unusably small team.
/// Per-turn bus and session failures are recorded in the returned status.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_cli_relay::Host>() -> anyhow::Result<()> {
/// let result = codetether_cli_relay::run_protocol_first_relay::<H>(2, "repair", "provider/model", None, None).await?;
/// # Ok(()) }
/// ```
pub async fn run_protocol_first_relay<H: crate::Host>(
    agent_count: usize,
    task: &str,
    model_ref: &str,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) -> Result<AutochatCliResult> {
    let mut runtime =
        crate::bootstrap::start::<H>(agent_count, task, model_ref, okr_id, okr_run_id).await?;
    runtime.drive(task, model_ref, okr_run_id).await;
    Ok(runtime.finish(model_ref, okr_run_id).await)
}
