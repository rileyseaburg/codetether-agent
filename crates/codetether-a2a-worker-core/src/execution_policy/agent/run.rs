use super::{AgentOutcome, Host};
use crate::execution_policy::{map_session_result, map_swarm_result};
/// Run the selected worker agent and normalize its result.
/// # Arguments
/// `host` supplies configured native execution; `agent_type` selects swarm or standard execution.
/// # Returns
/// Task status, result text, error and session identifier.
/// # Errors
/// Propagates the selected executor's error before result mapping.
/// # Examples
/// ```no_run
/// # async fn run(host: &mut impl codetether_a2a_worker_core::execution_policy::agent::Host) -> anyhow::Result<()> {
/// let _result = codetether_a2a_worker_core::execution_policy::agent::execute(host, "build").await?;
/// # Ok(()) }
/// ```
pub async fn execute(host: &mut impl Host, agent_type: &str) -> anyhow::Result<AgentOutcome> {
    if crate::is_swarm_agent(agent_type) {
        return host
            .swarm()
            .await
            .map(|(result, success)| map_swarm_result(result.text, success, result.session_id));
    }
    host.standard().await.map(|outcome| {
        map_session_result(
            outcome.result.text,
            outcome.result.session_id,
            outcome.budget_exhausted,
            outcome.max_steps,
        )
    })
}
